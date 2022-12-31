//! `PlotSpec` → Scene IR 编译（A2：`geom_line` + cartesian2d）。

use apollo_data::ColumnTable;
use apollo_scene::{AxisNode, CameraSpec, Point2, PolylineNode, Scene, SceneArena, SceneMetadata, SceneNodeKind, Viewport};
use apollo_types::{Diagnostic, DiagnosticCode, Interval, Result, Rgba};

use crate::{
    layer::{GeomSpec, LayerSpec},
    mapping::Mapping,
    plot::{DataRef, PlotSpec},
    scale::ScaleSpec,
    validate::validate_plot,
};

/// 编译选项。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompileOptions {
    /// 视口。
    pub viewport: Viewport,
    /// 绘图区内边距（左、下、右、上）。
    pub margin: (f64, f64, f64, f64),
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self { viewport: Viewport::new(640.0, 480.0), margin: (48.0, 36.0, 16.0, 16.0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PlotFrame {
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
}

impl PlotFrame {
    fn from_options(options: &CompileOptions) -> Self {
        let (left, bottom, right, top) = options.margin;
        Self { left, bottom, right: options.viewport.width - right, top: options.viewport.height - top }
    }

    fn width(self) -> f64 {
        self.right - self.left
    }

    fn height(self) -> f64 {
        self.top - self.bottom
    }
}

/// 校验并编译图规格为 Scene IR。
pub fn compile_plot(spec: &PlotSpec, options: CompileOptions) -> Result<Scene> {
    validate_plot(spec)?;
    let DataRef::Table(table) = &spec.data;
    let mut arena = SceneArena::new();
    let mut children = Vec::new();

    let frame = PlotFrame::from_options(&options);
    let x_domain = domain_for(table, &spec.mapping, &spec.layers, &spec.scales, "x")?;
    let y_domain = domain_for(table, &spec.mapping, &spec.layers, &spec.scales, "y")?;

    children.push(arena.insert(SceneNodeKind::Axis(AxisNode {
        horizontal: true,
        domain: x_domain,
        origin: Point2::new(frame.left, frame.bottom),
        length: frame.width(),
        tick_count: 5,
        stroke: Rgba::BLACK,
    })));
    children.push(arena.insert(SceneNodeKind::Axis(AxisNode {
        horizontal: false,
        domain: y_domain,
        origin: Point2::new(frame.left, frame.bottom),
        length: frame.height(),
        tick_count: 5,
        stroke: Rgba::BLACK,
    })));

    for layer in &spec.layers {
        match &layer.geom {
            GeomSpec::Line(line) => {
                let mapping = spec.mapping.merge(&layer.mapping);
                let points = project_line(table, &mapping, x_domain, y_domain, frame)?;
                children.push(arena.insert(SceneNodeKind::Polyline(PolylineNode {
                    points,
                    stroke: Rgba::BLACK,
                    linewidth: line.linewidth,
                })));
            }
        }
    }

    let root = arena.insert(SceneNodeKind::Group { children });
    Ok(Scene {
        root,
        nodes: arena,
        camera: CameraSpec { orthographic_2d: true },
        viewport: options.viewport,
        metadata: SceneMetadata::default(),
    })
}

fn domain_for(
    table: &ColumnTable,
    plot_mapping: &Mapping,
    layers: &[LayerSpec],
    scales: &[ScaleSpec],
    aesthetic: &str,
) -> Result<Interval> {
    if let Some(scale) = scales.iter().find(|scale| scale.aesthetic == aesthetic)
        && let Some(domain) = scale.domain
    {
        return Ok(domain);
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for layer in layers {
        let mapping = plot_mapping.merge(&layer.mapping);
        let name = match aesthetic {
            "x" => mapping.x.as_ref().map(|expr| expr.column_name()),
            "y" => mapping.y.as_ref().map(|expr| expr.column_name()),
            _ => None,
        }
        .ok_or_else(|| {
            Diagnostic::error(DiagnosticCode::MissingMapping, format!("缺少 {aesthetic} mapping"))
                .with_param("aesthetic", aesthetic)
        })?;
        let column = table.float_column(name)?;
        for value in &column.values {
            if value.is_finite() {
                min = min.min(*value);
                max = max.max(*value);
            }
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return Err(
            Diagnostic::error(DiagnosticCode::EmptyData, format!("{aesthetic} 域为空")).with_param("aesthetic", aesthetic)
        );
    }
    if (max - min).abs() < f64::EPSILON {
        max = min + 1.0;
    }
    Ok(Interval::new(min, max))
}

fn project_line(
    table: &ColumnTable,
    mapping: &Mapping,
    x_domain: Interval,
    y_domain: Interval,
    frame: PlotFrame,
) -> Result<Vec<Point2>> {
    let x_name =
        mapping.x.as_ref().ok_or_else(|| Diagnostic::error(DiagnosticCode::MissingMapping, "缺少 x mapping"))?.column_name();
    let y_name =
        mapping.y.as_ref().ok_or_else(|| Diagnostic::error(DiagnosticCode::MissingMapping, "缺少 y mapping"))?.column_name();
    let xs = &table.float_column(x_name)?.values;
    let ys = &table.float_column(y_name)?.values;
    if xs.len() != ys.len() {
        return Err(Diagnostic::error(DiagnosticCode::ColumnLengthMismatch, "x/y 列长度不一致"));
    }
    let mut points = Vec::with_capacity(xs.len());
    for (x, y) in xs.iter().zip(ys.iter()) {
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        points.push(Point2::new(
            map_range(*x, x_domain, frame.left, frame.right),
            map_range(*y, y_domain, frame.bottom, frame.top),
        ));
    }
    if points.len() < 2 {
        return Err(Diagnostic::error(DiagnosticCode::EmptyData, "折线至少需要两个有限点"));
    }
    Ok(points)
}

fn map_range(value: f64, domain: Interval, out_min: f64, out_max: f64) -> f64 {
    let t = (value - domain.min) / domain.span();
    out_min + t * (out_max - out_min)
}

#[cfg(test)]
mod tests {
    use apollo_data::ColumnTable;
    use apollo_scene::SceneNodeKind;

    use super::*;
    use crate::{layer::LayerSpec, mapping::Mapping};

    #[test]
    fn compiles_line_to_polyline_and_axes() {
        let table =
            ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![0.0, 1.0, 0.0]).unwrap();
        let plot = PlotSpec::new(table).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
        let scene = compile_plot(&plot, CompileOptions::default()).unwrap();
        let root = scene.nodes.get(scene.root).unwrap();
        let SceneNodeKind::Group { children } = &root.kind
        else {
            panic!("root must be group");
        };
        assert_eq!(children.len(), 3);
        assert!(matches!(scene.nodes.get(children[0]).unwrap().kind, SceneNodeKind::Axis(_)));
        assert!(matches!(scene.nodes.get(children[1]).unwrap().kind, SceneNodeKind::Axis(_)));
        assert!(matches!(scene.nodes.get(children[2]).unwrap().kind, SceneNodeKind::Polyline(_)));
    }
}

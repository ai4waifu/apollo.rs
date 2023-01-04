//! SVG 矢量后端。

use apollo_scene::{AxisNode, Point2, PolylineNode, Scene};
use apollo_types::{Diagnostic, DiagnosticCode, Result};

use crate::{
    Renderer,
    prepare::PreparedScene,
    report::{Capability, FrameReport, RendererCapabilities},
    target::{RenderTarget, color_to_css},
    walk::{Drawable, walk_drawables},
};

/// SVG 渲染器。
#[derive(Debug, Default, Clone, Copy)]
pub struct SvgRenderer;

impl SvgRenderer {
    /// 构造。
    pub const fn new() -> Self {
        Self
    }
}

impl Renderer for SvgRenderer {
    fn capabilities(&self) -> RendererCapabilities {
        RendererCapabilities { raster_2d: Capability::Unsupported, svg: Capability::Available, gpu: Capability::Unsupported }
    }

    fn prepare(&mut self, scene: &Scene) -> Result<PreparedScene> {
        Ok(PreparedScene::from_scene(scene))
    }

    fn render(&mut self, prepared: &PreparedScene, target: &mut RenderTarget) -> Result<FrameReport> {
        let RenderTarget::Svg(document) = target
        else {
            return Err(Diagnostic::error(DiagnosticCode::UnsupportedTarget, "SvgRenderer 需要 Svg 目标"));
        };

        let scene = &prepared.scene;
        let width = scene.viewport.width.max(1.0);
        let height = scene.viewport.height.max(1.0);
        let mut body = String::new();
        let mut primitive_count = 0_u32;

        walk_drawables(scene, |drawable| {
            match drawable {
                Drawable::Polyline(polyline) => {
                    primitive_count += 1;
                    body.push_str(&polyline_svg(polyline, height));
                }
                Drawable::Axis(axis) => {
                    let (axis_svg, count) = axis_svg(axis, height);
                    primitive_count += count;
                    body.push_str(&axis_svg);
                }
            }
            Ok(())
        })?;

        *document = format!(
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                "\n",
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#,
                "\n",
                r#"<rect width="100%" height="100%" fill="white"/>"#,
                "\n",
                "{body}",
                "</svg>\n"
            ),
            w = width,
            h = height,
            body = body
        );

        Ok(FrameReport { primitive_count })
    }
}

fn flip_y(point: Point2, height: f64) -> (f64, f64) {
    (point.x, height - point.y)
}

fn polyline_svg(polyline: &PolylineNode, height: f64) -> String {
    let points: Vec<String> = polyline
        .points
        .iter()
        .map(|point| {
            let (x, y) = flip_y(*point, height);
            format!("{x:.3},{y:.3}")
        })
        .collect();
    format!(
        r#"<polyline fill="none" stroke="{stroke}" stroke-width="{width}" points="{points}"/>"#,
        stroke = color_to_css(polyline.stroke),
        width = polyline.linewidth,
        points = points.join(" ")
    ) + "\n"
}

fn axis_svg(axis: &AxisNode, height: f64) -> (String, u32) {
    let origin = axis.origin;
    let end = if axis.horizontal {
        Point2::new(origin.x + axis.length, origin.y)
    }
    else {
        Point2::new(origin.x, origin.y + axis.length)
    };
    let (x0, y0) = flip_y(origin, height);
    let (x1, y1) = flip_y(end, height);
    let stroke = color_to_css(axis.stroke);
    let mut out = format!(r#"<line x1="{x0:.3}" y1="{y0:.3}" x2="{x1:.3}" y2="{y1:.3}" stroke="{stroke}" stroke-width="1"/>"#);
    out.push('\n');
    let mut count = 1_u32;

    let ticks = axis.tick_count.max(2);
    for i in 0..ticks {
        let t = f64::from(i) / f64::from(ticks - 1);
        let (start, end) = if axis.horizontal {
            let x = origin.x + axis.length * t;
            (Point2::new(x, origin.y), Point2::new(x, origin.y - 6.0))
        }
        else {
            let y = origin.y + axis.length * t;
            (Point2::new(origin.x, y), Point2::new(origin.x - 6.0, y))
        };
        let (sx, sy) = flip_y(start, height);
        let (ex, ey) = flip_y(end, height);
        out.push_str(&format!(
            r#"<line x1="{sx:.3}" y1="{sy:.3}" x2="{ex:.3}" y2="{ey:.3}" stroke="{stroke}" stroke-width="1"/>"#
        ));
        out.push('\n');
        count += 1;
    }
    (out, count)
}

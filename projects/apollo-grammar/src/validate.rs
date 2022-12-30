//! `PlotSpec` 校验（A1：2D `geom_line`）。

use apollo_data::ColumnTable;
use apollo_types::{Diagnostic, DiagnosticCode, Result};

use crate::{
    coordinate::CoordinateSpec,
    layer::{GeomSpec, StatSpec},
    mapping::Mapping,
    plot::{DataRef, PlotSpec},
    scale::ScaleKind,
};

/// 校验图规格是否可进入后续编译。
pub fn validate_plot(spec: &PlotSpec) -> Result<()> {
    let DataRef::Table(table) = &spec.data;
    table.validate()?;

    if spec.layers.is_empty() {
        return Err(Diagnostic::error(DiagnosticCode::InvalidLayer, "至少需要一个图层"));
    }

    match spec.coordinates {
        CoordinateSpec::Cartesian2d => {}
    }

    for scale in &spec.scales {
        match scale.kind {
            ScaleKind::Continuous => {}
        }
    }

    for (index, layer) in spec.layers.iter().enumerate() {
        if layer.data.is_some() {
            return Err(Diagnostic::error(DiagnosticCode::UnsupportedSpec, "A1 尚未支持层级覆盖数据")
                .with_param("layer", index.to_string()));
        }
        match layer.stat {
            StatSpec::Identity => {}
        }
        match &layer.geom {
            GeomSpec::Line(_) => validate_line_layer(table, &spec.mapping, &layer.mapping, index)?,
        }
    }

    Ok(())
}

fn validate_line_layer(table: &ColumnTable, plot_mapping: &Mapping, layer_mapping: &Mapping, layer_index: usize) -> Result<()> {
    let mapping = plot_mapping.merge(layer_mapping);
    let x = mapping.x.as_ref().ok_or_else(|| {
        Diagnostic::error(DiagnosticCode::MissingMapping, "geom_line 需要 x mapping")
            .with_param("layer", layer_index.to_string())
            .with_param("aesthetic", "x")
    })?;
    let y = mapping.y.as_ref().ok_or_else(|| {
        Diagnostic::error(DiagnosticCode::MissingMapping, "geom_line 需要 y mapping")
            .with_param("layer", layer_index.to_string())
            .with_param("aesthetic", "y")
    })?;
    table.float_column(x.column_name())?;
    table.float_column(y.column_name())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use apollo_data::ColumnTable;

    use super::*;
    use crate::{layer::LayerSpec, mapping::Mapping};

    fn sample_table() -> ColumnTable {
        ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![0.0, 1.0, 0.0]).unwrap()
    }

    #[test]
    fn accepts_line_plot() {
        let plot = PlotSpec::new(sample_table()).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
        assert!(validate_plot(&plot).is_ok());
    }

    #[test]
    fn rejects_missing_y() {
        let mut mapping = Mapping::default();
        mapping.x = Some(crate::mapping::AestheticExpr::column("x"));
        let plot = PlotSpec::new(sample_table()).mapping(mapping).layer(LayerSpec::geom_line());
        let err = validate_plot(&plot).unwrap_err();
        assert_eq!(err.code, DiagnosticCode::MissingMapping);
    }

    #[test]
    fn rejects_unknown_column() {
        let plot = PlotSpec::new(sample_table()).mapping(Mapping::xy("x", "missing")).layer(LayerSpec::geom_line());
        let err = validate_plot(&plot).unwrap_err();
        assert_eq!(err.code, DiagnosticCode::UnknownColumn);
    }
}

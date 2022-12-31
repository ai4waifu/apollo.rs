//! 门面级冒烟：构造并校验 2D 折线图规格。

use apollo::{ColumnTable, LayerSpec, Mapping, PlotSpec, validate_plot};

#[test]
fn facade_validates_line_plot() {
    let table = ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![1.0, 2.0, 3.0]).unwrap();
    let plot = PlotSpec::new(table).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
    validate_plot(&plot).unwrap();
}

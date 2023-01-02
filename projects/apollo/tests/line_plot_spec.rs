//! 门面级冒烟：构造、校验并编译 2D 折线图。

use apollo::{ColumnTable, CompileOptions, LayerSpec, Mapping, PlotSpec, SceneNodeKind, compile_plot, validate_plot};

#[test]
fn facade_validates_line_plot() {
    let table = ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![1.0, 2.0, 3.0]).unwrap();
    let plot = PlotSpec::new(table).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
    validate_plot(&plot).unwrap();
}

#[test]
fn facade_compiles_line_plot_to_scene() {
    let table = ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![1.0, 2.0, 3.0]).unwrap();
    let plot = PlotSpec::new(table).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
    let scene = compile_plot(&plot, CompileOptions::default()).unwrap();
    let root = scene.nodes.get(scene.root).unwrap();
    assert!(matches!(root.kind, SceneNodeKind::Group { .. }));
}

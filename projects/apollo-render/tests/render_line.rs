//! CPU / SVG 渲染冒烟。

use apollo_data::ColumnTable;
use apollo_grammar::{CompileOptions, LayerSpec, Mapping, PlotSpec, compile_plot};
use apollo_render::{render_rgba8, render_svg};
use apollo_scene::SceneNodeKind;

fn sample_scene() -> apollo_scene::Scene {
    let table = ColumnTable::new().push_float("x", vec![0.0, 1.0, 2.0]).unwrap().push_float("y", vec![0.0, 2.0, 1.0]).unwrap();
    let plot = PlotSpec::new(table).mapping(Mapping::xy("x", "y")).layer(LayerSpec::geom_line());
    compile_plot(&plot, CompileOptions::default()).unwrap()
}

#[test]
fn cpu_raster_draws_non_white_pixels() {
    let scene = sample_scene();
    let image = render_rgba8(&scene).unwrap();
    assert_eq!(image.width, 640);
    assert_eq!(image.height, 480);
    assert!(image.non_white_count() > 100, "expected drawn primitives");
}

#[test]
fn svg_contains_polyline_and_axes() {
    let scene = sample_scene();
    let svg = render_svg(&scene).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("<polyline"));
    assert!(svg.matches("<line").count() >= 2);

    let root = scene.nodes.get(scene.root).unwrap();
    assert!(matches!(root.kind, SceneNodeKind::Group { .. }));
}

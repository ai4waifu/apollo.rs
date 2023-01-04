//! Apollo 公共 Rust 门面 — 分层绘图 crate 的稳定入口。
//!
//! ```text
//! 数据 / PlotSpec → grammar → scene → layout / render → CPU | SVG |（可选）WGPU
//! ```

#![deny(missing_docs)]

pub use apollo_data::{Column, ColumnTable, FloatColumn};
pub use apollo_grammar::{
    AestheticExpr, CompileOptions, CoordinateSpec, DataRef, GeomLine, GeomSpec, LayerParameters, LayerSpec, Mapping, PlotSpec,
    PositionSpec, ScaleKind, ScaleSpec, StatSpec, ThemeSpec, compile_plot, validate_plot,
};
pub use apollo_render::{
    Capability, CpuRasterRenderer, FrameReport, PreparedScene, RenderTarget, Renderer, RendererCapabilities, RgbaImage,
    SvgRenderer, render_rgba8, render_svg,
};
pub use apollo_scene::{
    AxisNode, CameraSpec, Point2, PolylineNode, Scene, SceneArena, SceneMetadata, SceneNode, SceneNodeId, SceneNodeKind,
    Viewport,
};
pub use apollo_types::{Diagnostic, DiagnosticCode, Interval, NodeId, Result, Rgba, SerializationVersion, Severity};

#[cfg(feature = "wgpu")]
pub use apollo_backend_wgpu as backend_wgpu;

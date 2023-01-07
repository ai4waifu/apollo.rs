//! Apollo 公共 Rust 门面 — 分层绘图 crate 的稳定入口。
//!
//! ```text
//! 数据 / PlotSpec → grammar → scene → layout / render
//!   → CPU |（默认）SVG |（可选）WGPU
//! ```

#![deny(missing_docs)]

pub use apollo_data::{
    Column, ColumnTable, FloatColumn, GraphData, GraphEdge, GraphNode, GridData, StringColumn, TreeData, TreeEdge, TreeNode,
};
pub use apollo_grammar::{
    AestheticExpr, Camera3dSpec, Cartesian2d, Cartesian3d, CompileOptions, CoordinateSpec, DataRef, FacetSpec, GeomBar,
    GeomEdge, GeomLine, GeomNode, GeomPoint, GeomPoint3d, GeomSpec, GeomSurface, GeomText, GeomTreeEdge, GeomTreeNode,
    GraphLayoutKind, GraphSpace, LayerParameters, LayerSpec, Mapping, PlotSpec, PositionSpec, ScaleKind, ScaleSpec, StatSpec,
    ThemeSpec, TreeLayoutKind, TreeSpace, compile_plot, validate_plot,
};
pub use apollo_layout::{
    CircularLayout, EdgeRoute, ForceLayout, GraphLayout, GridLayout, LayeredLayout, LayoutOptions, LayoutPoint, LayoutResult,
    PanelRect, RadialTreeLayout, TidyTreeLayout, TreeLayout, layout_facet_panels, layout_single_panel, straight_routes,
};
pub use apollo_render::{
    Capability, CpuRasterRenderer, FrameReport, PreparedScene, RenderTarget, Renderer, RendererCapabilities, RgbaImage,
    render_rgba8,
};
pub use apollo_scene::{
    AxisNode, CameraSpec, Mesh3Node, MeshNode, Point2, Points3Node, PointsNode, PolylineNode, Ray, Rect2, Scene, SceneArena,
    SceneMetadata, SceneNode, SceneNodeId, SceneNodeKind, ScreenPoint, TextNode, Viewport, pick_at, project_to_screen,
    screen_to_ray, try_project_to_screen,
};
pub use apollo_types::{
    Diagnostic, DiagnosticCode, HitResult, InteractionId, Interval, NodeId, PrimitiveId, Result, Rgba, RowId,
    SerializationVersion, Severity, Vec3,
};

#[cfg(feature = "svg")]
pub use apollo_backend_svg::{SvgRenderer, render_svg};

#[cfg(feature = "wgpu")]
pub use apollo_backend_wgpu::{WgpuRenderer, render_rgba8_wgpu};

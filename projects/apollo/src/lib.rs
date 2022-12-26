//! Apollo 公共 Rust 门面 — 分层绘图 crate 的稳定入口。
//!
//! ```text
//! 数据 / PlotSpec → grammar → scene → layout / render → CPU | SVG |（可选）WGPU
//! ```

#![deny(missing_docs)]

pub use apollo_data::CRATE_NAME as DATA_CRATE;
pub use apollo_grammar::CRATE_NAME as GRAMMAR_CRATE;
pub use apollo_layout::CRATE_NAME as LAYOUT_CRATE;
pub use apollo_render::CRATE_NAME as RENDER_CRATE;
pub use apollo_scene::CRATE_NAME as SCENE_CRATE;
pub use apollo_types::CRATE_NAME as TYPES_CRATE;

/// A0 workspace 冻结用的 crate 占位标记。
pub const CRATE_NAME: &str = "apollo";

#[cfg(feature = "wgpu")]
pub use apollo_backend_wgpu::CRATE_NAME as WGPU_CRATE;

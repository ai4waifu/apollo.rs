//! Apollo 渲染器合同 — CPU、SVG、GPU 后端共用。
//!
//! 只消费 Scene IR。不得重新推断 stat、scale 或 layout。

#![deny(missing_docs)]

/// A0 workspace 冻结用的 crate 占位标记。
pub const CRATE_NAME: &str = "apollo-render";

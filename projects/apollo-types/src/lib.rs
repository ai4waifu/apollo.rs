//! Apollo 共享类型合同 — ID、颜色、单位、范围、诊断。
//!
//! 不包含数据表、布局或 GPU 代码。

#![deny(missing_docs)]

mod color;
mod diagnostic;
mod hit;
mod ids;
mod range;
mod vec3;

pub use color::Rgba;
pub use diagnostic::{Diagnostic, DiagnosticCode, Result, Severity};
pub use hit::HitResult;
pub use ids::{InteractionId, NodeId, PrimitiveId, RowId, SerializationVersion};
pub use range::Interval;
pub use vec3::Vec3;

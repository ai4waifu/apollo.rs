//! Apollo 共享类型合同 — ID、颜色、单位、范围、诊断。
//!
//! 不包含数据表、布局或 GPU 代码。

#![deny(missing_docs)]

mod color;
mod diagnostic;
mod ids;
mod range;

pub use color::Rgba;
pub use diagnostic::{Diagnostic, DiagnosticCode, Result, Severity};
pub use ids::{NodeId, SerializationVersion};
pub use range::Interval;

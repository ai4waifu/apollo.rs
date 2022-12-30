//! Apollo 列式数据合同 — 表、视图、缺失值、批次。
//!
//! 不含图形语法或数学求值。

#![deny(missing_docs)]

mod column;
mod table;

pub use column::{Column, FloatColumn};
pub use table::ColumnTable;

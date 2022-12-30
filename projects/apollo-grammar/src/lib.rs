//! Apollo Grammar of Graphics — `PlotSpec`、mapping、stat、geom、scale、coord、facet、theme。
//!
//! 向 Scene IR 编译。不绘制像素，也不调用设备 API。

#![deny(missing_docs)]

mod coordinate;
mod layer;
mod mapping;
mod plot;
mod scale;
mod validate;

pub use coordinate::CoordinateSpec;
pub use layer::{GeomLine, GeomSpec, LayerParameters, LayerSpec, PositionSpec, StatSpec};
pub use mapping::{AestheticExpr, Mapping};
pub use plot::{DataRef, PlotSpec, ThemeSpec};
pub use scale::{ScaleKind, ScaleSpec};
pub use validate::validate_plot;

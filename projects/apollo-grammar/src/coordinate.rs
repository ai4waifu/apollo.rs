//! Coordinate 合同。与 Scale 独立组合。

/// 坐标系规格。
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum CoordinateSpec {
    /// 二维笛卡尔。
    #[default]
    Cartesian2d,
}

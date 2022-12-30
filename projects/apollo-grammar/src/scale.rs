//! Scale 合同。Scale 与 Coordinate 分离。

use apollo_types::Interval;

/// Scale 种类。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScaleKind {
    /// 连续线性。
    Continuous,
}

/// 单个 scale 规格。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScaleSpec {
    /// 绑定的 aesthetic 名，例如 `x` / `y`。
    pub aesthetic: String,
    /// 种类。
    pub kind: ScaleKind,
    /// 可选显式 domain；`None` 表示由数据推断。
    pub domain: Option<Interval>,
}

impl ScaleSpec {
    /// 连续 x scale。
    pub fn continuous_x() -> Self {
        Self { aesthetic: "x".into(), kind: ScaleKind::Continuous, domain: None }
    }

    /// 连续 y scale。
    pub fn continuous_y() -> Self {
        Self { aesthetic: "y".into(), kind: ScaleKind::Continuous, domain: None }
    }
}

//! 图层、stat、geom、position。

use crate::mapping::Mapping;

/// 统计变换（A1：identity）。
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum StatSpec {
    /// 原样传递。
    #[default]
    Identity,
}

/// 折线几何参数。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeomLine {
    /// 线宽（场景单位，后续由 theme/scale 细化）。
    pub linewidth: f32,
}

impl Default for GeomLine {
    fn default() -> Self {
        Self { linewidth: 1.0 }
    }
}

/// 几何标记。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GeomSpec {
    /// 折线。
    Line(GeomLine),
}

impl GeomSpec {
    /// 默认折线。
    pub fn line() -> Self {
        Self::Line(GeomLine::default())
    }
}

/// 位置调整（A1：identity）。
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum PositionSpec {
    /// 不调整。
    #[default]
    Identity,
}

/// 图层附加参数（占位，避免把常量伪装成 mapping）。
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct LayerParameters {}

/// 单层图形规格。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LayerSpec {
    /// 可选覆盖数据键。`None` 表示沿用图级数据。
    pub data: Option<String>,
    /// 层内 mapping（与图级合并）。
    pub mapping: Mapping,
    /// 统计变换。
    pub stat: StatSpec,
    /// 几何。
    pub geom: GeomSpec,
    /// 位置调整。
    pub position: PositionSpec,
    /// 常量参数。
    pub parameters: LayerParameters,
}

impl LayerSpec {
    /// 折线层。
    pub fn geom_line() -> Self {
        Self {
            data: None,
            mapping: Mapping::default(),
            stat: StatSpec::Identity,
            geom: GeomSpec::line(),
            position: PositionSpec::Identity,
            parameters: LayerParameters::default(),
        }
    }

    /// 覆盖层 mapping。
    pub fn mapping(mut self, mapping: Mapping) -> Self {
        self.mapping = mapping;
        self
    }
}

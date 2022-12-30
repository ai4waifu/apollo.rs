//! `PlotSpec` 与主题占位。

use apollo_data::ColumnTable;

use crate::{coordinate::CoordinateSpec, layer::LayerSpec, mapping::Mapping, scale::ScaleSpec};

/// 图级数据引用。A1 直接内嵌 `ColumnTable`。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataRef {
    /// 列式表。
    Table(ColumnTable),
}

/// 主题占位（A3 再充实）。
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ThemeSpec {}

/// 声明式图形规格。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlotSpec {
    /// 主数据。
    pub data: DataRef,
    /// 图级 mapping。
    pub mapping: Mapping,
    /// 图层。
    pub layers: Vec<LayerSpec>,
    /// Scale 列表。
    pub scales: Vec<ScaleSpec>,
    /// 坐标系。
    pub coordinates: CoordinateSpec,
    /// 主题。
    pub theme: ThemeSpec,
}

impl PlotSpec {
    /// 由列式表构造空白图规格（尚无图层）。
    pub fn new(table: ColumnTable) -> Self {
        Self {
            data: DataRef::Table(table),
            mapping: Mapping::default(),
            layers: Vec::new(),
            scales: vec![ScaleSpec::continuous_x(), ScaleSpec::continuous_y()],
            coordinates: CoordinateSpec::Cartesian2d,
            theme: ThemeSpec::default(),
        }
    }

    /// 设置图级 mapping。
    pub fn mapping(mut self, mapping: Mapping) -> Self {
        self.mapping = mapping;
        self
    }

    /// 追加图层。
    pub fn layer(mut self, layer: LayerSpec) -> Self {
        self.layers.push(layer);
        self
    }

    /// 设置坐标系。
    pub fn coordinates(mut self, coordinates: CoordinateSpec) -> Self {
        self.coordinates = coordinates;
        self
    }
}

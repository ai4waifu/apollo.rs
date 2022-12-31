//! 场景节点种类与几何载荷（A2 首切片）。

use apollo_types::{Interval, NodeId, Rgba};

/// 二维点（场景空间）。
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Point2 {
    /// x。
    pub x: f64,
    /// y。
    pub y: f64,
}

impl Point2 {
    /// 构造点。
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 折线节点。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PolylineNode {
    /// 折线顶点（已是场景坐标）。
    pub points: Vec<Point2>,
    /// 描边色。
    pub stroke: Rgba,
    /// 线宽。
    pub linewidth: f32,
}

/// 坐标轴节点（A2：线性刻度轴）。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AxisNode {
    /// 是否为水平轴（x）。否则为竖直轴（y）。
    pub horizontal: bool,
    /// 数据域。
    pub domain: Interval,
    /// 轴在场景中的起点。
    pub origin: Point2,
    /// 轴长度（场景单位）。
    pub length: f64,
    /// 刻度数量（含两端）。
    pub tick_count: u32,
    /// 描边色。
    pub stroke: Rgba,
}

/// 场景节点种类。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SceneNodeKind {
    /// 分组。
    Group {
        /// 子节点。
        children: Vec<NodeId>,
    },
    /// 折线。
    Polyline(PolylineNode),
    /// 坐标轴。
    Axis(AxisNode),
}

/// Arena 中的场景节点。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneNode {
    /// 稳定 ID。
    pub id: NodeId,
    /// 种类与载荷。
    pub kind: SceneNodeKind,
}

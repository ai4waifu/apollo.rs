//! 稳定标识与版本标记。

/// Scene / 资源节点稳定 ID（arena 友好）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct NodeId(pub u64);

/// 序列化合同版本。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SerializationVersion(pub u32);

impl SerializationVersion {
    /// 当前合同版本。
    pub const CURRENT: Self = Self(1);
}

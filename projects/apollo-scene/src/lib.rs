//! Apollo Scene IR — 唯一的后端无关渲染合同。
//!
//! Arena + 稳定 ID。不含 ggplot 语义，也不含设备 / GPU API。

#![deny(missing_docs)]

mod arena;
mod node;
mod scene;

pub use apollo_types::NodeId as SceneNodeId;
pub use arena::SceneArena;
pub use node::{AxisNode, Point2, PolylineNode, SceneNode, SceneNodeKind};
pub use scene::{CameraSpec, Scene, SceneMetadata, Viewport};

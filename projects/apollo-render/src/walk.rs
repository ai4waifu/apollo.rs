//! 遍历 Scene 树并回调可绘制节点。

use apollo_scene::{AxisNode, PolylineNode, Scene, SceneNodeKind};
use apollo_types::{Diagnostic, DiagnosticCode, NodeId, Result};

/// 可绘制图元。
pub(crate) enum Drawable<'a> {
    /// 折线。
    Polyline(&'a PolylineNode),
    /// 坐标轴。
    Axis(&'a AxisNode),
}

/// 前序遍历，对每个可绘制节点调用 `visitor`。
pub(crate) fn walk_drawables(scene: &Scene, mut visitor: impl FnMut(Drawable<'_>) -> Result<()>) -> Result<()> {
    walk_node(scene, scene.root, &mut visitor)
}

fn walk_node(scene: &Scene, id: NodeId, visitor: &mut impl FnMut(Drawable<'_>) -> Result<()>) -> Result<()> {
    let node = scene.nodes.get(id).ok_or_else(|| {
        Diagnostic::error(DiagnosticCode::RenderFailed, format!("缺失场景节点 {}", id.0)).with_param("node", id.0.to_string())
    })?;
    match &node.kind {
        SceneNodeKind::Group { children } => {
            for child in children {
                walk_node(scene, *child, visitor)?;
            }
        }
        SceneNodeKind::Polyline(polyline) => visitor(Drawable::Polyline(polyline))?,
        SceneNodeKind::Axis(axis) => visitor(Drawable::Axis(axis))?,
    }
    Ok(())
}

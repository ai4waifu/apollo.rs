#![allow(missing_docs)]

use apollo_data::{TreeData, TreeNode};
use apollo_types::DiagnosticCode;

#[test]
fn accepts_small_tree() {
    let tree = TreeData::new(
        "r",
        vec![TreeNode::root("r"), TreeNode::child("a", "r"), TreeNode::child("b", "r"), TreeNode::child("c", "a")],
    )
    .unwrap();
    assert_eq!(tree.edges().len(), 3);
}

#[test]
fn rejects_forest() {
    let err = TreeData::new("r", vec![TreeNode::root("r"), TreeNode::root("orphan")]).unwrap_err();
    assert_eq!(err.code, DiagnosticCode::ValidationFailed);
}

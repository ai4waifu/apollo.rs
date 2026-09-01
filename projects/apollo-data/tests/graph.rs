#![allow(missing_docs)]

use apollo_data::{GraphData, GraphEdge, GraphNode};
use apollo_types::DiagnosticCode;

#[test]
fn rejects_unknown_endpoint() {
    let err = GraphData::undirected(vec![GraphNode::new("a")], vec![GraphEdge::new("a", "b")]).unwrap_err();
    assert_eq!(err.code, DiagnosticCode::UnknownColumn);
}

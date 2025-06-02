use dater::Graph;

#[test]
fn test_create_empty_graph() {
    // Arrange
    let graph: Graph<i32> = Graph::new();

    // Assert
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_add_node() {
    // Arrange
    let mut graph: Graph<i32> = Graph::new();

    // Act
    let node_id = graph.insert(42).unwrap();

    // Assert
    assert_eq!(graph.node_count(), 1);
    assert!(graph.node(node_id).is_some());
    assert_eq!(**graph.node(node_id).unwrap(), 42);
}

#[test]
fn test_add_edge() {
    // Arrange
    let mut graph: Graph<i32, &str> = Graph::new();
    let node1 = graph.insert(1).unwrap();
    let node2 = graph.insert(2).unwrap();

    // Act
    let edge = graph.connect(node1, "connects", node2).unwrap();

    // Assert
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.edge(edge).is_some());
    assert_eq!(**graph.edge(edge).unwrap(), "connects");
}

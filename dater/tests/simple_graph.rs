use dater::Graph;

#[test]
fn test_create_empty_graph() {
    // Arrange
    let graph = Graph::new();

    // Assert
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_add_node() {
    // Arrange
    let mut graph = Graph::new();

    // Act
    let node_id = graph.add_node(42);

    // Assert
    assert_eq!(graph.node_count(), 1);
    assert!(graph.contains_node(node_id));
    assert_eq!(graph.get_node(node_id), Some(&42));
}

#[test]
fn test_add_edge() {
    // Arrange
    let mut graph = Graph::new();
    let node1 = graph.add_node(1);
    let node2 = graph.add_node(2);

    // Act
    let edge_id = graph.add_edge(node1, node2, "connects");

    // Assert
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.contains_edge(edge_id));
    assert_eq!(graph.get_edge(edge_id), Some(&"connects"));
}

#[test]
fn test_fluent_add() {
    // Arrange
    let mut graph = Graph::new();

    let first_node = graph.add_node(10);
    graph.add_node(20).add_edge(first_node, "link");

    // Assert
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.contains_node(0));
    assert!(graph.contains_node(1));
    assert!(graph.contains_edge(0));
    assert_eq!(graph.get_edge(0), Some(&"link"));
}

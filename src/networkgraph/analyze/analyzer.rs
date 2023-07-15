use petgraph::algo::dijkstra;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;

use crate::networkgraph::table::types::{DataError, TableEntry};
use crate::networkgraph::table::{generic_network::NodeData, openrank_network::OpenRankNode};

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error("NodeNotFound")]
    NodeNotFound,
    #[error("PathNotFound")]
    NoPathFound,
    #[error("NoEdgesFound")]
    NoEdgesFound,
}

pub(crate) async fn dijkstra_open_rank_node(
    node_a: &str,
    node_b: &str,
    graph: Graph<OpenRankNode, f64, Undirected>,
) -> Result<String, AnalyzeError> {
    let node_a_index = match graph
        .node_indices()
        .find(|&index| graph[index].id == node_a)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let node_b_index = match graph
        .node_indices()
        .find(|&index| graph[index].id == node_b)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let shortest_path = dijkstra(&graph, node_a_index, Some(node_b_index), |e| *e.weight());

    if let Some(length) = shortest_path.get(&node_b_index) {
        let path = shortest_path
            .iter()
            .filter(|(_, &dist)| (dist - *length).abs() < 1e-6)
            .map(|(&index, _)| graph[index].id.clone())
            .collect::<Vec<String>>()
            .join(" -> ");

        Ok(format!("route: {} -> {}", path, graph[node_a_index].id))
    } else {
        Err(AnalyzeError::NoPathFound)
    }
}

pub(crate) async fn dijkstra_node(
    node_a: &str,
    node_b: &str,
    graph: Graph<NodeData, f64, Undirected>,
) -> Result<String, AnalyzeError> {
    let node_a_index = match graph
        .node_indices()
        .find(|&index| graph[index].name == node_a)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let node_b_index = match graph
        .node_indices()
        .find(|&index| graph[index].name == node_b)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let shortest_path = dijkstra(&graph, node_a_index, Some(node_b_index), |e| *e.weight());

    if let Some(length) = shortest_path.get(&node_b_index) {
        let path = shortest_path
            .iter()
            .filter(|(_, &dist)| (dist - *length).abs() < 1e-6)
            .map(|(&index, _)| graph[index].name.clone())
            .collect::<Vec<String>>()
            .join(" -> ");

        Ok(format!("route: {} -> {}", path, graph[node_a_index].name))
    } else {
        Err(AnalyzeError::NoPathFound)
    }
}

pub(crate) async fn edge_from_open_rank_node(
    node: &str,
    graph: Graph<OpenRankNode, f64, Undirected>,
) -> Result<String, AnalyzeError> {
    let node_index = match graph.node_indices().find(|&index| graph[index].id == node) {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let edges = graph
        .edges(node_index)
        .map(|edge| {
            let source_id = graph[EdgeRef::source(&edge)].id.clone();
            let target_id = graph[EdgeRef::target(&edge)].id.clone();
            format!("{} -> {}", source_id, target_id)
        })
        .collect::<Vec<String>>()
        .join(", ");

    Ok(edges)
}

pub(crate) async fn edge_from_node(
    node: &str,
    graph: Graph<NodeData, f64, Undirected>,
) -> Result<String, AnalyzeError>{
    let node_index = match graph.node_indices().find(|&index| graph[index].name == node) {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let edges = graph
        .edges(node_index)
        .map(|edge| {
            let source_id = graph[EdgeRef::source(&edge)].name.clone();
            let target_id = graph[EdgeRef::target(&edge)].name.clone();
            format!("{} -> {}", source_id, target_id)
        })
        .collect::<Vec<String>>()
        .join(", ");

    Ok(edges)
}

pub(crate) async fn get_neighbors_from_open_rank_node(
    node: &str,
    graph: &Graph<OpenRankNode, f64, Undirected>,
) -> Result<Vec<String>, AnalyzeError> {
    let node_index = match graph.node_indices().find(|&index| graph[index].id == node) {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let neighbors = graph
        .neighbors(node_index)
        .map(|neighbor_index| graph[neighbor_index].id.clone())
        .collect();

    Ok(neighbors)
}

pub(crate) async fn get_neighbors_from_node(
    node: &str,
    graph: &Graph<NodeData, f64, Undirected>,
) -> Result<Vec<String>, AnalyzeError> {
    let node_index = match graph.node_indices().find(|&index| graph[index].name == node) {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let neighbors = graph
        .neighbors(node_index)
        .map(|neighbor_index| graph[neighbor_index].name.clone())
        .collect();

    Ok(neighbors)
}

pub(crate) async fn get_value_from_open_rank_node(
    node: &str,
    graph: &Graph<OpenRankNode, f64, Undirected>,
) -> Result<f64, AnalyzeError> {
    let node_index = match graph
        .node_indices()
        .find(|&index| graph[index].id == node)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let value = graph[node_index].v;

    Ok(value)
}


pub(crate) async fn get_value_from_node(
    node: &str,
    graph: Graph<NodeData, f64, Undirected>,
) -> Result<f64, AnalyzeError> {
    let node_index = match graph
        .node_indices()
        .find(|&index| graph[index].name == node)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let value = graph[node_index].weight;

    Ok(value)
}


pub(crate) async fn get_biggest_edge_node_from_open_rank_node(
    node: &str,
    graph: Graph<OpenRankNode, f64, Undirected>,
) -> Result<String, AnalyzeError> {
    let node_index = match graph
        .node_indices()
        .find(|&index| graph[index].id == node)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let max_edge_weight = graph.edges(node_index)
        .map(|edge| *edge.weight())
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if let Some(max_weight) = max_edge_weight {
        let max_edge_node_indices: Vec<petgraph::stable_graph::NodeIndex> = graph.edges(node_index)
            .filter(|edge| *edge.weight() == max_weight)
            .map(|edge| edge.target())
            .collect();

        let max_edge_node_ids: Vec<String> = max_edge_node_indices.iter()
            .map(|&index| graph[index].id.clone())
            .collect();

        Ok(max_edge_node_ids.join(", "))
    } else {
        Err(AnalyzeError::NoEdgesFound)
    }
}


pub(crate) async fn get_biggest_edge_node_from_node(
    node: &str,
    graph: Graph<NodeData, f64, Undirected>,
) -> Result<String, AnalyzeError> {
    let node_index = match graph
        .node_indices()
        .find(|&index| graph[index].name == node)
    {
        Some(index) => index,
        None => return Err(AnalyzeError::NodeNotFound),
    };

    let max_edge_weight = graph.edges(node_index)
        .map(|edge| *edge.weight())
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if let Some(max_weight) = max_edge_weight {
        let max_edge_node_indices: Vec<petgraph::stable_graph::NodeIndex> = graph.edges(node_index)
            .filter(|edge| *edge.weight() == max_weight)
            .map(|edge| edge.target())
            .collect();

        let max_edge_node_ids: Vec<String> = max_edge_node_indices.iter()
            .map(|&index| graph[index].name.clone())
            .collect();

        Ok(max_edge_node_ids.join(", "))
    } else {
        Err(AnalyzeError::NoEdgesFound)
    }
}




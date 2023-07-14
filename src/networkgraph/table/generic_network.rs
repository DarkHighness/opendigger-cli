use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::Undirected;

use crate::networkgraph::table::types::TableOwner;

use crate::api::{
   ApiError, Metric,
};

use super::types::DataError;

#[derive(Debug, Clone, serde::Deserialize)]
struct Network {
    nodes: Vec<(String, f64)>,
    edges: Vec<(String, String, f64)>,
}

#[derive(Debug)]
pub struct NodeData {
    pub name: String,
    pub weight: f64,
}

fn create_graph(data: Network) -> Graph<NodeData, f64, Undirected> {

    let mut graph: Graph<NodeData, f64, Undirected> = Graph::new_undirected();
    let mut node_indices: Vec<NodeIndex> = Vec::new();

    for node in data.nodes.iter() {
        let node_message = NodeData{
            name : node.0.to_string(),
            weight : node.1
        };
    
        let node_index = graph.add_node(node_message);
        node_indices.push(node_index);
    }
    for edge in data.edges.iter() {
        let source_label: String = edge.0.to_string();
        let target_label: String = edge.1.to_string();
        let weight: f64 = edge.2;

        let source_index = node_indices.iter().find(|&&i| graph[i].name == source_label).unwrap();
        let target_index = node_indices.iter().find(|&&i| graph[i].name == target_label).unwrap();

        graph.add_edge(*source_index, *target_index, weight);
    } 


    graph
}

pub(crate) async fn fetch_network_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Graph<NodeData, f64, Undirected>, DataError>{  
    let api = crate::api::get();

    let mut graph: Graph<NodeData, f64, Undirected> = Graph::new_undirected();
    let data_result: Result<Network, ApiError> = api
        .get::<Network>(owner.to_string().as_str(), *metric)
        .await;
    match data_result {
        Ok(data) => {
            graph = create_graph(data);
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }

    Ok(graph)

}


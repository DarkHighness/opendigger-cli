use petgraph::Graph;
use petgraph::graph::NodeIndex;
use serde_json::Value;
use petgraph::Undirected;

use crate::networkgraph::table::types::TableOwner;

use crate::api::{
   ApiError, Metric,
};

use super::types::DataError;

#[derive(Debug, Clone, serde::Deserialize)]
struct Network {
    nodes: Vec<NetworkNode>,
    links: Vec<Link>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct NetworkNode {
    id: String,
    n: Value,
    c: String,
    i: f64,
    r: f64,
    v: f64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Link {
    s: String,
    t: String,
    w: f64,
}

pub struct OpenRankNode {
    pub id: String,
    pub n: String,
    pub c: String,
    pub i: f64,
    pub r: f64,
    pub v: f64,
}

fn create_graph(data: Network) -> Graph<OpenRankNode, f64, Undirected> {
    let mut graph: Graph<OpenRankNode, f64, Undirected> = Graph::new_undirected();
    let mut node_indices: Vec<NodeIndex> = Vec::new();

    for node in data.nodes.iter() {
        let node_message = OpenRankNode{
            id : node.id.to_string(),
            n : node.n.to_string(),
            c : node.c.to_string(),
            i : node.i,
            r : node.r,
            v : node.v,
        };
    
        let node_index = graph.add_node(node_message);
        node_indices.push(node_index);
    }
    for edge in data.links.iter() {
        let source_label: String = edge.s.to_string();
        let target_label: String = edge.t.to_string();
        let weight: f64 = edge.w;

        let source_index = node_indices.iter().find(|&&i| graph[i].id == source_label).unwrap();
        let target_index = node_indices.iter().find(|&&i| graph[i].id == target_label).unwrap();

        graph.add_edge(*source_index, *target_index, weight);
    } 


    graph
}

pub(crate) async fn fetch_network_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Graph<OpenRankNode, f64, Undirected>, DataError>{  
    let api = crate::api::get();

    let mut graph: Graph<OpenRankNode, f64, Undirected> = Graph::new_undirected();
    let data_result: Result<Network, ApiError>  = api
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


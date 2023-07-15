use self::analyzer::{
    dijkstra_node, dijkstra_open_rank_node, edge_from_node, edge_from_open_rank_node,
    get_biggest_edge_node_from_node, get_biggest_edge_node_from_open_rank_node,
    get_neighbors_from_node, get_neighbors_from_open_rank_node, get_value_from_node,
    get_value_from_open_rank_node, AnalyzeError,
};

use super::table::generic_network::NodeData;
use super::table::openrank_network::OpenRankNode;
use super::table::types::{DataError, TableEntry};

use petgraph::Graph;
use petgraph::Undirected;

mod analyzer;

pub(crate) async fn get_neighbors(
    metric: &str,
    owner: &str,
    node: &str,
) -> Result<Vec<String>, AnalyzeError> {
    if metric == "OpenRank" {
        let table_entry = TableEntry::parse("OpenRank", &owner).unwrap();
        let data_result: Result<Graph<OpenRankNode, f64, Undirected>, DataError> =
            table_entry.fetch_openrank_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<Vec<String>, AnalyzeError> =
                    get_neighbors_from_open_rank_node(node, &graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    } else {
        let table_entry = TableEntry::parse("DeveloperNetwork", &owner).unwrap();
        let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> =
            table_entry.fetch_generic_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<Vec<String>, AnalyzeError> =
                    get_neighbors_from_node(node, &graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    }
}

pub(crate) async fn get_max_neighbor(
    metric: &str,
    owner: &str,
    node: &str,
) -> Result<String, AnalyzeError> {
    if metric == "OpenRank" {
        let table_entry = TableEntry::parse("OpenRank", &owner).unwrap();
        let data_result: Result<Graph<OpenRankNode, f64, Undirected>, DataError> =
            table_entry.fetch_openrank_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<String, AnalyzeError> =
                    get_biggest_edge_node_from_open_rank_node(node, graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    } else {
        println!("{},{}",owner,node);
        let table_entry = TableEntry::parse("DeveloperNetwork", owner).unwrap();
        let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> =
            table_entry.fetch_generic_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<String, AnalyzeError> =
                    get_biggest_edge_node_from_node(node, graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    }
}

pub(crate) async fn get_node_value(
    metric: &str,
    owner: &str,
    node: &str,
) -> Result<f64, AnalyzeError> {
    if metric == "OpenRank" {
        let table_entry = TableEntry::parse("OpenRank", &owner).unwrap();
        let data_result: Result<Graph<OpenRankNode, f64, Undirected>, DataError> =
            table_entry.fetch_openrank_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<f64, AnalyzeError> =
                    get_value_from_open_rank_node(node, &graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    } else {
        let table_entry = TableEntry::parse("DeveloperNetwork", &owner).unwrap();
        let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> =
            table_entry.fetch_generic_network_data().await;

        match data_result {
            Ok(graph) => {
                let data: Result<f64, AnalyzeError> = get_value_from_node(node, graph).await;
                match data {
                    Ok(result) => Ok(result),
                    Err(error) => Err(AnalyzeError::NoPathFound),
                }
            }
            Err(error) => {
                println!("Error fetching network data: {:?}", error);
                Err(AnalyzeError::NodeNotFound)
            }
        }
    }
}

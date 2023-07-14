use self::analyzer::{dijkstra_node, AnalyzeError, dijkstra_open_rank_node, get_biggest_edge_node_from_open_rank_node, get_biggest_edge_node_from_node, get_value_from_node};

use super::table::generic_network::NodeData;
use super::table::openrank_network::OpenRankNode;
use super::table::types::{TableEntry, DataError};

use petgraph::Graph;
use petgraph::Undirected;

mod analyzer;


pub(crate) async fn test()
{
    let table_entry = TableEntry::parse("DeveloperNetwork", "frank-zsy").unwrap();
    let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> = table_entry.fetch_generic_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<String, AnalyzeError> = dijkstra_node("Zzzzzhuzhiwei", "will-ww", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }

    let table_entry = TableEntry::parse("RepoNetwork", "X-lab2017/open-digger").unwrap();
    let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> = table_entry.fetch_generic_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<String, AnalyzeError> = dijkstra_node("sathishcyberintelsys/skf-labsss", "NOUIY/aws-sdk-java", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }

    let table_entry = TableEntry::parse("SpecialCircumstances", "X-lab2017/open-digger").unwrap();
    let data_result: Result<Graph<OpenRankNode, f64, Undirected>, DataError> = table_entry.fetch_openrank_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<String, AnalyzeError> = dijkstra_open_rank_node("44269", "101833", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }


    let table_entry = TableEntry::parse("SpecialCircumstances", "X-lab2017/open-digger").unwrap();
    let data_result: Result<Graph<OpenRankNode, f64, Undirected>, DataError> = table_entry.fetch_openrank_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<String, AnalyzeError> = get_biggest_edge_node_from_open_rank_node("44269", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }

    let table_entry = TableEntry::parse("DeveloperNetwork", "X-lab2017/open-digger").unwrap();
    let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> = table_entry.fetch_generic_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<String, AnalyzeError> = get_biggest_edge_node_from_node("Zzzzzhuzhiwei", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }

    let table_entry = TableEntry::parse("DeveloperNetwork", "X-lab2017/open-digger").unwrap();
    let data_result: Result<Graph<NodeData, f64, Undirected>, DataError> = table_entry.fetch_generic_network_data().await;

    match data_result {
        Ok(graph) => {
            let data:Result<f64, AnalyzeError> = get_value_from_node("Zzzzzhuzhiwei", graph).await;
            match data {
                Ok(result) => println!("Result: {}", result),
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(error) => {
            println!("Error fetching network data: {:?}", error);
        }
    }


}
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::borrow::Cow;
use regex::Regex;

use crate::api::RepositoryMetric;

use super::ReportError;


#[derive(Debug, Clone, serde::Deserialize)]
pub struct NetworkGraph {
    nodes: Vec<(String, f64)>,
    edges: Vec<(String, String, f64)>,
}

pub struct RepoOverview {
    pub owner: String,
    pub star_trend: Option<Vec<(String, i64)>>,
    pub repo_network: Option<NetworkGraph>,
}

impl RepoOverview {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            star_trend: None,
            repo_network: None,
        }
    }

    pub async fn generate_report(&mut self) -> Result<(), ReportError> {
        let api = crate::api::get();
        let star_trend = api
            .get::<BTreeMap<String, i64>>(&self.owner, RepositoryMetric::Stars.into())
            .await?
            .into_iter()
            .fold(Vec::new(), |state, element| {
                let mut state = state;
                if let Some((_, last_value)) = state.last() {
                    let (date, value) = element;
                    state.push((date, value + last_value));
                } else {
                    state.push(element);
                }
                state
            });

        let dates1: Vec<String> = star_trend
            .iter()
            .map(|(string, _)| format!("\"{}\"", string))
            .collect::<Vec<String>>();
        let numbers1: Vec<i64> = star_trend.iter().map(|(_, number)| *number).collect();
        let numbers_str1 = format!("[{}]", numbers1.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));
        let dates_str1 = format!("[{}]", dates1.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));

        let fork_trend = api
            .get::<BTreeMap<String, i64>>(&self.owner, RepositoryMetric::TechnicalFork.into())
            .await?
            .into_iter()
            .fold(Vec::new(), |state, element| {
                let mut state = state;
                if let Some((_, last_value)) = state.last() {
                    let (date, value) = element;
                    state.push((date, value + last_value));
                } else {
                    state.push(element);
                }
                state
            });

        let dates2: Vec<String> = fork_trend
            .iter()
            .map(|(string, _)| format!("\"{}\"", string))
            .collect::<Vec<String>>();
        let numbers2: Vec<i64> = fork_trend.iter().map(|(_, number)| *number).collect();
        let numbers_str2 = format!("[{}]", numbers2.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));
        let dates_str2 = format!("[{}]", dates2.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));

        
        let openrank_trend = api
            .get::<BTreeMap<String, f64>>(&self.owner, RepositoryMetric::OpenRank.into())
            .await?
            .into_iter()
            .fold(Vec::new(), |state, element| {
                let mut state = state;
                if let Some((_, last_value)) = state.last() {
                    let (date, value) = element;
                    state.push((date, value + last_value));
                } else {
                    state.push(element);
                }
                state
            });

        let dates3: Vec<String> = openrank_trend
            .iter()
            .map(|(string, _)| format!("\"{}\"", string))
            .collect::<Vec<String>>();
        let numbers3: Vec<f64> = openrank_trend.iter().map(|(_, number)| *number).collect();
        let numbers_str3 = format!("[{}]", numbers3.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));
        let dates_str3 = format!("[{}]", dates3.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));


        let activity_trend = api
            .get::<BTreeMap<String, f64>>(&self.owner, RepositoryMetric::Activity.into())
            .await?
            .into_iter()
            .fold(Vec::new(), |state, element| {
                let mut state = state;
                if let Some((_, last_value)) = state.last() {
                    let (date, value) = element;
                    state.push((date, value + last_value));
                } else {
                    state.push(element);
                }
                state
            });

        let dates4: Vec<String> = activity_trend
            .iter()
            .map(|(string, _)| format!("\"{}\"", string))
            .collect::<Vec<String>>();
        let numbers4: Vec<f64> = activity_trend.iter().map(|(_, number)| *number).collect();
        let numbers_str4 = format!("[{}]", numbers4.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));
        let dates_str4 = format!("[{}]", dates4.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "));


        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("src");
        file_path.push("report");
        file_path.push("scripts");
        file_path.push("quota.js");

        let mut file = OpenOptions::new().create(true)
            .read(true).write(true).open(file_path)?;
        let reader = BufReader::new(file);
        let contents: Vec<String> = reader.lines()
            .map(|line| line.expect("Failed to read line"))
            .map(|line| line.into())
            .collect();

        let _repo_network = api
        .get::<NetworkGraph>(&self.owner, RepositoryMetric::RepoNetwork.into())
        .await?;
        let nodes_string: String = _repo_network
        .nodes
        .iter()
        .map(|(name, value)| format!("[\"{}\", {}]", name, value))
        .collect::<Vec<String>>()
        .join(", ");
        let _nodes_string = format!("[{}]", nodes_string);

        let edges_string: String = _repo_network
        .edges
        .iter()
        .map(|(name1, name2, value)| format!("[\"{}\",\"{}\", {}]", name1, name2, value))
        .collect::<Vec<String>>()
        .join(", ");
        let _edges_string = format!("[{}]", edges_string);


        // 使用正则表达式找到 var data1 后面的内容
        let re0 = Regex::new(r#"var\s+data1\s+=\s+(.*?);"#).unwrap();
        let replaced_contents0: Vec<String> = contents.iter()
            .map(|line| {
                let line: Cow<str> = line.into();
                re0.replace(line.as_ref(), format!("var data1 = {};", dates_str1))
                .into_owned()
            })
            .collect();
        let re1 = Regex::new(r#"var\s+data2\s+=\s+(.*?);"#).unwrap();
            let replaced_contents1: Vec<String> = replaced_contents0.iter()
            .map(|line| {
                let line: Cow<str> = line.into();
                re1.replace(line.as_ref(), format!("var data2 = {};", numbers_str1))
                .into_owned()
            })
            .collect();

        let re2 = Regex::new(r#"var\s+data3\s+=\s+(.*?);"#).unwrap();
        let replaced_contents2: Vec<String> = replaced_contents1.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re2.replace(line.as_ref(), format!("var data3 = {};", dates_str2))
            .into_owned()
        })
        .collect();

        let re3 = Regex::new(r#"var\s+data4\s+=\s+(.*?);"#).unwrap();
        let replaced_contents3: Vec<String> = replaced_contents2.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re3.replace(line.as_ref(), format!("var data4 = {};", numbers_str2))
            .into_owned()
        })
        .collect();

        let re4 = Regex::new(r#"var\s+data5\s+=\s+(.*?);"#).unwrap();
        let replaced_contents4: Vec<String> = replaced_contents3.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re4.replace(line.as_ref(), format!("var data5 = {};", dates_str3))
            .into_owned()
        })
        .collect();

        let re5 = Regex::new(r#"var\s+data6\s+=\s+(.*?);"#).unwrap();
        let replaced_contents5: Vec<String> = replaced_contents4.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re5.replace(line.as_ref(), format!("var data6 = {};", numbers_str3))
            .into_owned()
        })
        .collect();

        let re6 = Regex::new(r#"var\s+data7\s+=\s+(.*?);"#).unwrap();
        let replaced_contents6: Vec<String> = replaced_contents5.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re6.replace(line.as_ref(), format!("var data7 = {};", dates_str4))
            .into_owned()
        })
        .collect();

        let re7 = Regex::new(r#"var\s+data8\s+=\s+(.*?);"#).unwrap();
        let replaced_contents7: Vec<String> = replaced_contents6.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re7.replace(line.as_ref(), format!("var data8 = {};", numbers_str4))
            .into_owned()
        })
        .collect();

        let re8 = Regex::new(r#"var\s+data9\s+=\s+(.*?);"#).unwrap();
        let replaced_contents8: Vec<String> = replaced_contents7.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re8.replace(line.as_ref(), format!("var data9 = {};", _nodes_string))
            .into_owned()
        })
        .collect();

        let re9 = Regex::new(r#"var\s+data10\s+=\s+(.*?);"#).unwrap();
        let replaced_contents: Vec<String> = replaced_contents8.iter()
        .map(|line| {
            let line: Cow<str> = line.into();
            re9.replace(line.as_ref(), format!("var data10 = {};", _edges_string))
            .into_owned()
        })
        .collect();


    // 将替换后的内容写回文件
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("src");
        file_path.push("report");
        file_path.push("scripts");
        file_path.push("quota.js");
    let file = File::create(&file_path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);
    for line in replaced_contents {
        writer.write_all(line.as_bytes()).expect("Failed to write line");
        writer.write_all(b"\n").expect("Failed to write newline");
    }

        Ok(())
        
    }
}

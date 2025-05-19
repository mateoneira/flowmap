use std::collections::HashMap;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum GraphError {
    #[error("unknown node id")]
    UnknownNodeId,
    #[error("invalid coordinate")]
    InvalidCoordinate,
}


#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Edge {
    pub origin: u64,
    pub dest: u64,
    pub od_id: u64,
    pub counts: i64,
    pub distance: f64,
    pub locked: std::sync::atomic::AtomicBool,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    index: HashMap<u64, usize>,
}



use dashmap::DashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct State {
    pub resources_map: DashMap<String, ResourceStorage>,
}

#[derive(Debug, Clone)]
pub struct ResourceStorage {
    pub transaction_path: PathBuf,
}

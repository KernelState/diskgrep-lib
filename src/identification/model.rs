use crate::disk::model::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdItem {
    pub has_directory: Vec<String>,
    pub has_format: Option<String>,
    pub in_disk: Option<String>,
}

impl IdItem {
    pub fn new(directory: Vec<String>, format: Option<String>, in_disk: Option<String>) -> Self {
        Self {
            has_directory: directory,
            has_format: format,
            in_disk: in_disk,
        }
    }
}

#[derive(Debug)]
pub struct Id {
    pub not: bool,
    pub dir_and: bool,
    pub all_and: bool,
    pub id: IdItem,
    pub met: bool,
    pub candidates: Vec<Partition>,
}

impl Id {
    pub fn new(id: IdItem, not: bool, dir_and: bool, all_and: bool) -> Self {
        Self {
            not: not,
            dir_and: dir_and,
            all_and: all_and,
            id: id,
            met: false,
            candidates: Vec::new(),
        }
    }
}

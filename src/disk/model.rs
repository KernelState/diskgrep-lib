use crate::disk::units::Size;

pub trait Partitionable {}

#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub size: Size,
    pub fstype: String,
    pub uuid: String,
    pub mountpoint: Option<String>,
    pub mounted: bool,
}

impl Partitionable for Partition {}

impl Partition {
    pub fn new(name: String, size: Size) -> Self {
        Self {
            name: name,
            size: size,
            fstype: String::from("Unknown"),
            uuid: String::from("Unknown"),
            mountpoint: None,
            mounted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub serial: String,
    pub model: String,
    pub size: Size,
    pub children: Vec<Partition>,
}

impl Partitionable for Disk {}

impl Disk {
    pub fn new(name: String, size: Size) -> Self {
        Self {
            name: name,
            serial: String::from("Unknown"),
            model: String::from("Unknown"),
            size: size,
            children: Vec::new(),
        }
    }
}

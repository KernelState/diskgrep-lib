use crate::disk::model::{Disk, Partition};
use crate::disk::scanner::load_list;
use crate::utils::error::{Error, ErrorKind};

pub enum DiskTypes {
    Disk(Disk),
    Partition(Partition),
}

pub fn find_part_in_root(name: String) -> Result<DiskTypes, Error> {
    let disk_list = match load_list() {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    for i in disk_list {
        if i.name == name {
            return Ok(DiskTypes::Disk(i));
        }
        for p in i.children {
            if p.name == name {
                return Ok(DiskTypes::Partition(p));
            }
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        format!("\"{name}\" not found"),
    ))
}

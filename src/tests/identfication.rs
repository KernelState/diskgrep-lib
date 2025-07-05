use crate::disk::model::Disk;
use crate::disk::scanner::load_list;
use crate::identification::identification::find;
use crate::identification::model::{Id, IdItem};

// I have a windows partition on my PC so im just testing for this case
#[test]
fn find_windows_disk() {
    let mut windows_id = Id::new(
        IdItem::new(
            vec![String::from("Windows")],
            Option::Some(String::from("ntfs")),
            Option::None,
        ),
        false,
        true,
        true,
    );

    find(&mut windows_id, true);
    assert_eq!(windows_id.candidates.len(), 0 as usize);
}

// this looks for everything that is not my projects_disk
#[test]
fn find_not_projects() {
    let disk_list: Vec<Disk> = match load_list() {
        Err(e) => panic!("{}", e),
        Ok(v) => v,
    };
    let mut not_proj = Id::new(
        IdItem::new(
            vec![String::from("Projects")],
            Some(String::from("exfat")),
            Option::None,
        ),
        true,
        true,
        false,
    );
    find(&mut not_proj, true);

    let mut disk_ammount: usize = 0;

    // listen im lazy to change it to .len() if it already works
    for i in disk_list {
        for p in i.children {
            disk_ammount += 1;
        }
    }

    assert_eq!(not_proj.candidates.len() < disk_ammount - 1, true);
}

use crate::disk::model::Partition;
use crate::disk::mount::{mount, umount};
use crate::utils::find::{find_part_in_root, DiskTypes};
use std::env::home_dir;

// if it gives a directory not found error then everything is fine but
// you just need sudo perms to mkdir in /mnt as a default dir

// sda3 is what is in my case, its ntfs it has windows on it (should work tho it did before)
#[test]
fn mount_test() {
    let mut part: Partition = match find_part_in_root(String::from("sda3")) {
        Err(e) => panic!("{e}"),
        Ok(v) => match v {
            DiskTypes::Disk(_) => panic!("Disk found??"),
            DiskTypes::Partition(v) => v,
        },
    };
    let mut v = String::new();
    if let Option::Some(a) = home_dir() {
        v = a.into_os_string().into_string().unwrap();
    } else {
        panic!("home directory not found (not a mounting error), please check the environment used to test");
    }
    mount(&mut part, false, false, true, format!("{v}/.diskgrep_test")).unwrap();
    umount(&mut part).unwrap();
}

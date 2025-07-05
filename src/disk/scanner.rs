use crate::disk::model::{Disk, Partition};
use crate::disk::parser;
use crate::disk::units::Size;
use crate::utils::callers::Cmd;
use crate::utils::error::Error;

pub fn load_list() -> Result<Vec<Disk>, Error> {
    let cmd_obj = Cmd::new(String::from(
        "lsblk -J -o NAME,FSTYPE,SIZE,MOUNTPOINT,UUID,SERIAL,MODEL",
    ));
    let output = match cmd_obj.run() {
        Err(e) => return Err(e.excec()),
        Ok(v) => match parser::parse_root(v.as_str()) {
            Err(e) => return Err(e.excec()),
            Ok(v) => v,
        },
    };
    let mut arrangement: Vec<Disk> = Vec::new();
    for i in output {
        if i.name.contains("loop") {
            continue;
        }
        let mut children: Vec<Partition> = Vec::new();
        for b in i.children {
            let fstype = match b.fstype {
                None => String::from("Unknown"),
                Some(v) => v,
            };
            let uuid = match b.uuid {
                None => String::from("Unknown"),
                Some(v) => v,
            };
            children.push(Partition {
                name: b.name,
                size: Size::from_str(b.size.as_str()).unwrap(),
                fstype: fstype,
                uuid: uuid,
                mounted: !parser::is_null::<String>(&b.mountpoint),
                mountpoint: b.mountpoint,
            })
        }
        let serial = match i.serial {
            None => String::from("Unknown"),
            Some(v) => v,
        };
        let model = match i.model {
            None => String::from("Unknown"),
            Some(v) => v,
        };
        arrangement.push(Disk {
            name: i.name,
            serial: serial,
            model: model,
            size: Size::from_str(i.size.as_str()).unwrap(),
            children: children,
        });
    }
    Ok(arrangement)
}

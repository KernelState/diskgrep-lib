use crate::disk::mount::{mount, umount};
use crate::disk::scanner::load_list;
use crate::identification::model::Id;
use std::fs;
use std::path::Path;

fn log(msg: String, debug: &bool) {
    if debug.clone() {
        println!("{msg}")
    }
}

// this is what a never netser does every day, every second
pub fn find(id: &mut Id, debug: &bool) {
    for d in load_list().unwrap() {
        for mut p in d.children {
            log(format!("found {p:?}"), debug);
            let remove = !p.mounted;
            if remove {
                log(
                    format!("{} is not mounted, mounting temporarly", p.name),
                    debug,
                );
                if p.fstype != String::from("Unknown") {
                    match mount(&mut p, false, false, false, String::new()) {
                        Err(e) => {
                            log(format!("Silent Error, skipping partition"), debug);
                            continue;
                        }
                        Ok(_) => {}
                    }
                } else {
                    log(
                        format!("skipping partition {} with unrecogniseable fstype", p.name),
                        debug,
                    );
                    continue;
                }
            }
            log(
                format!("partition is mounted in {}", p.mountpoint.clone().unwrap()),
                debug,
            );
            let mut conds = 0;
            let mut met = 0;
            if id.id.has_directory.len() > 0 {
                conds += 1;
                let mut dirs_found: u32 = 0;
                log(
                    format!("required directories = {:?}", id.id.has_directory),
                    debug,
                );
                for dir in id.id.has_directory.clone() {
                    log(
                        format!(
                            "directories are being compared to {}/{dir}",
                            p.mountpoint.clone().unwrap()
                        ),
                        debug,
                    );
                    for dirname in fs::read_dir(p.mountpoint.clone().unwrap())
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .map(|e| e.path().to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                    {
                        log(format!("found dir {dirname} in {}", p.name), debug);
                        if dirname == format!("{}/{dir}", p.mountpoint.clone().unwrap()) {
                            dirs_found += 1;
                        }
                    }
                    log(format!("found {dirs_found} dirs"), debug);
                    let old_met = met.clone();
                    match id.dir_and {
                        true => {
                            if (dirs_found as usize) == id.id.has_directory.len() {
                                met += 1;
                            }
                        }
                        false => {
                            if (dirs_found as usize) > 0 {
                                met += 1;
                            }
                        }
                    }
                    if met > old_met {
                        println!("{} has met the directory condition", p.name);
                    }
                }
            }
            if let Option::Some(ref f) = id.id.has_format {
                conds += 1;
                log(
                    format!("comparing fstype {f} to partition's {}", p.fstype),
                    debug,
                );
                if *f == p.fstype {
                    met += 1;
                    println!("has met the fstype condition");
                }
            }
            if let Option::Some(ref t) = id.id.in_disk {
                conds += 1;
                log(format!("comparing disks name {t} to {}", d.name), debug);
                if *t == d.name {
                    met += 1;
                    println!("{} has me the inside condition", p.name);
                }
            }
            if remove {
                log(format!("unmounting temporarly mounted partition"), debug);
                match umount(&mut p) {
                    Err(e) => log(format!("Silent error"), debug),
                    Ok(_) => {}
                }
            }
            log(
                format!("filtring the met {met} conditions to {conds}"),
                debug,
            );
            match (id.all_and, id.not) {
                (true, false) => {
                    if met == conds {
                        id.candidates.push(p);
                    }
                }
                (false, false) => {
                    if met > 0 {
                        id.candidates.push(p);
                    }
                }
                (true, true) => {
                    if met == 0 {
                        id.candidates.push(p);
                    }
                }
                (false, true) => {
                    if met != conds {
                        id.candidates.push(p);
                    }
                }
            }
        }
    }
}

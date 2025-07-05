use crate::identification::model::{Id, IdItem};
use crate::utils::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    name: String,
    all_and: bool,
    dir_and: bool,
    id: IdItem,
    not: bool,
}

impl Tag {
    pub fn to_id(&self) -> Id {
        Id {
            candidates: Vec::new(),
            not: self.not.clone(),
            all_and: self.all_and.clone(),
            dir_and: self.dir_and.clone(),
            id: self.id.clone(),
            met: false,
        }
    }
    pub fn from_id(name: String, id: &Id) -> Self {
        Self {
            id: id.id.clone(),
            not: id.not.clone(),
            all_and: id.all_and.clone(),
            dir_and: id.dir_and.clone(),
            name: name.clone(),
        }
    }
}

pub fn save(name: String, dir_path: Option<String>, id: Id) -> Result<(), Error> {
    let mut dir: String = format!(
        "{}/.diskgrep",
        home_dir().expect("No home dir found").display()
    );
    if let Option::Some(v) = dir_path {
        dir = v;
    }
    println!("dir is {dir}");
    let pth = Path::new(dir.as_str());
    if !pth.exists() {
        fs::create_dir_all(&pth);
    }
    let file_name = format!("{dir}/{name}.json");
    let file_path = Path::new(file_name.as_str());
    if file_path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("file {dir}/{name}.json already exists"),
        ));
    }
    let mut file = match fs::File::create(format!("{dir}/{name}.json").as_str()) {
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("got an error {e}"))),
        Ok(v) => v,
    };
    let json_output = match serde_json::to_string(&Tag::from_id(name, &id)) {
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("got an error {e}"))),
        Ok(v) => v,
    };
    match file.write_all(Vec::from_iter(json_output.bytes()).as_slice()) {
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to write to file {e}"),
            ))
        }
        Ok(_) => {}
    }
    Ok(())
}

pub fn read(file_path: String) -> Result<Id, Error> {
    if !Path::new(file_path.as_str()).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("file {file_path} is not found"),
        ));
    }
    let file = match fs::read_to_string(file_path.as_str()) {
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("{e}"))),
        Ok(v) => v,
    };
    match serde_json::from_str::<Tag>(file.as_str()) {
        Err(e) => Err(Error::new(ErrorKind::Other, format!("error {e}"))),
        Ok(v) => Ok(v.to_id()),
    }
}

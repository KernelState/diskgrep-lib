use crate::utils::error::{Error, ErrorKind};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct LsblkElement {
    pub name: String,

    #[serde(default)]
    pub fstype: Option<String>,

    #[serde(default)]
    pub serial: Option<String>,

    #[serde(default)]
    pub model: Option<String>,

    #[serde(default)]
    pub uuid: Option<String>,
    pub size: String,

    #[serde(default)]
    pub mountpoint: Option<String>,

    #[serde(default)]
    pub children: Vec<LsblkElement>,
}

pub fn is_null<T>(v: &Option<T>) -> bool {
    return match v {
        None => true,
        Some(_) => false,
    };
}

pub fn parse_root(root_json: &str) -> Result<Vec<LsblkElement>, Error> {
    let json_block = match serde_json::from_str(root_json) {
        Err(e) => {
            return Err(Error::new(
                ErrorKind::ParsingError,
                format!("unable to parse json output due to : {e}"),
            ));
        }
        Ok(v) => v,
    };
    let mut elements: Vec<LsblkElement> = Vec::new();
    match json_block {
        serde_json::Value::Object(v) => {
            let element_list = v["blockdevices"].clone();
            match element_list {
                serde_json::Value::Array(arr) => {
                    for i in arr.iter() {
                        let element: Result<LsblkElement, _> =
                            serde_json::from_str(i.to_string().as_str());
                        match element {
                            Ok(v) => elements.push(v),
                            Err(e) => {
                                return Err(Error::new(
                                    ErrorKind::ParsingError,
                                    format!("cannot parse json due to {e}"),
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::ParsingError,
                        format!("the serde_json parser cannot parse children of blockdevices list"),
                    ));
                }
            }
            Ok(elements)
        }
        other => {
            return Err(Error::new(
                ErrorKind::UnknownType,
                format!("unknown json type {other}"),
            ));
        }
    }
}

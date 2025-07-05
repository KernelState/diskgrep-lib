use serde::Deserialize;

use crate::utils::error::{Error, ErrorKind};

#[derive(Debug, Clone, Deserialize)]
pub enum Size {
    Eib(f64),
    Pib(f64),
    Tib(f64),
    Gib(f64),
    Mib(f64),
    Kib(f64),
    Byte(u64),
}

impl Size {
    pub fn from_str(v: &str) -> Result<Self, Error> {
        // conerts from &str to chars() and then gets the last element and unwraps it then it turns
        // into a string then into an str again for ease when doing the matches
        let unit_char_string = String::from(v.chars().last().unwrap());
        let unit_char: &str = &unit_char_string.as_str();
        let nums: f64 = match String::from(&v[..v.len() - 1]).as_str().parse::<f64>() {
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::ParsingError,
                    format!(
                        "input \"{}\" into size was not a number, {e}",
                        String::from_iter(Vec::from_iter(v.chars()).pop().iter()).as_str()
                    ),
                ));
            }
            Ok(v) => v,
        };
        match unit_char {
            "E" => Ok(Self::Eib(nums)),
            "P" => Ok(Self::Pib(nums)),
            "T" => Ok(Self::Tib(nums)),
            "G" => Ok(Self::Gib(nums)),
            "M" => Ok(Self::Mib(nums)),
            "K" => Ok(Self::Kib(nums)),
            "B" => Ok(Self::Byte(nums as u64)),
            other => Err(Error::new(
                ErrorKind::UnknownType,
                format!("Unknown unit {other}"),
            )),
        }
    }
    pub fn to_str(&self) -> String {
        match self {
            Self::Eib(v) => format!("{v}E"),
            Self::Pib(v) => format!("{v}P"),
            Self::Tib(v) => format!("{v}T"),
            Self::Gib(v) => format!("{v}G"),
            Self::Mib(v) => format!("{v}M"),
            Self::Kib(v) => format!("{v}K"),
            Self::Byte(v) => format!("{v}B"),
        }
    }
}

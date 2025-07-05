use crate::utils::error::{Error, ErrorKind};
use std::process::Command;

pub struct Cmd {
    pub cmd_text: String,
    pub ran: bool,
}

impl Cmd {
    pub fn new(cmd_text: String) -> Self {
        Self {
            cmd_text: cmd_text,
            ran: false,
        }
    }
    pub fn run(&self) -> Result<String, Error> {
        if self.ran {
            return Err(Error::new(
                ErrorKind::AlreadyRan,
                String::from("cannot run a command twice"),
            ));
        }
        let parsed_input = Vec::from_iter(self.cmd_text.split_whitespace());
        let mut cmd = Command::new(parsed_input[0]);
        cmd.args(Vec::from_iter(parsed_input[1..].iter()));

        let output = match cmd.output() {
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::CommandExitedWithError,
                    format!("the command generated an unhandled error"),
                ));
            }
            Ok(v) => v,
        };
        let stderr = match String::from_utf8(output.stderr) {
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::EncodingError,
                    String::from("Cannot encode stderr"),
                ));
            }
            Ok(v) => v,
        };
        if !stderr.is_empty() {
            return Err(Error::new(
                ErrorKind::CommandExitedWithError,
                format!("the command generated an error: {stderr}"),
            ));
        }
        let stdout = match String::from_utf8(output.stdout) {
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::EncodingError,
                    String::from("Cannot encode stderr"),
                ));
            }
            Ok(v) => v,
        };
        Ok(stdout)
    }
}

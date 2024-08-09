use std::{
    io::{Error, Read, Result, Write},
    net::TcpStream,
};

use super::ADBCommand;

pub struct ADBShell {
    stream: TcpStream,
}

impl ADBShell {
    pub fn new(stream: TcpStream) -> ADBShell {
        ADBShell { stream }
    }

    pub fn exec(&mut self, cmd: &str) -> Result<String> {
        let cmd = ADBCommand::new(format!("shell:{}", cmd));
        self.stream.write_all(cmd.format_cmd().as_bytes())?;

        // status
        let mut buf = [0; 4];
        self.stream.read_exact(&mut buf)?;

        if String::from_utf8_lossy(&buf) == "OKAY" {
            let mut resopne = Vec::new();
            self.stream.read_to_end(&mut resopne)?;
            Ok(String::from_utf8_lossy(&resopne).to_string())
        } else {
            Err(Error::other("Failed to exec command"))
        }
    }
}

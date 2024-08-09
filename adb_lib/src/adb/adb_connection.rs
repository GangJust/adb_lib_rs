use std::{
    io::{Error, Read, Result, Write},
    net::TcpStream,
};

use super::{ADBCommand, ADBResponse, ADBShellResponse};

pub struct ADBConnection {
    pub ip: String,
    pub port: u16,
}

impl ADBConnection {
    /// create a new ADBConnection
    ///
    /// [ip] is the ip address of the adb server
    /// [port] is the port of the adb server
    pub fn new(ip: &str, port: u16) -> ADBConnection {
        ADBConnection {
            ip: ip.to_string(),
            port,
        }
    }

    /// build a ip:port string
    fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    /// establish a connection to the adb server
    fn connect(&self) -> Result<TcpStream> {
        TcpStream::connect(self.addr())
    }

    /// adb version
    pub fn version(&self) -> Result<ADBResponse> {
        let cmd = ADBCommand::new("host:version");
        let mut stream = self.connect()?;
        stream.write_all(cmd.format().as_bytes())?;

        ADBResponse::with(&stream)
    }

    /// adb devices -l
    pub fn devices(&self) -> Result<ADBResponse> {
        let cmd = ADBCommand::new("host:devices-l");
        let mut stream = self.connect()?;
        stream.write_all(cmd.format().as_bytes())?;

        ADBResponse::with(&stream)
    }

    /// adb -s <serial_no> get-state
    pub fn state<T>(&self, serial_no: T) -> Result<ADBResponse>
    where
        T: AsRef<str>,
    {
        let mut stream = self.connect()?;
        let cmd = ADBCommand::new(match serial_no.as_ref() {
            "" => "host:get-state".to_string(),
            _ => format!("host-serial:{}:get-state", serial_no.as_ref()).to_string(),
        });
        stream.write_all(cmd.format().as_bytes())?;

        ADBResponse::with(&stream)
    }

    /// adb -s <serial_no>
    fn select<T>(&self, serial_no: T) -> Result<TcpStream>
    where
        T: AsRef<str>,
    {
        let mut stream = self.connect()?;
        let cmd = ADBCommand::new(match serial_no.as_ref() {
            "" => "host:transport-any".to_string(),
            _ => format!("host:transport:{}", serial_no.as_ref()),
        });

        stream.write_all(cmd.format().as_bytes())?;

        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;

        if String::from_utf8_lossy(&buf) == "OKAY" {
            Ok(stream)
        } else {
            Err(Error::other("Failed to select device"))
        }
    }

    /// adb -s <serial_no> shell <sh>
    ///
    /// [serial_no] is empty, select any device
    ///
    /// This function establishes a connection to the specified device or any device if [serial_no] is empty.
    /// It returns a `TcpStream` that can be used for further communication with the selected device.
    pub fn shell<T>(&self, serial_no: T, sh: T) -> Result<ADBShellResponse>
    where
        T: AsRef<str>,
    {
        let mut stream = self.select(serial_no)?;

        let cmd = ADBCommand::new(format!("shell:{}", sh.as_ref()));
        stream.write_all(cmd.format().as_bytes())?;

        ADBShellResponse::with(&stream)
    }
}

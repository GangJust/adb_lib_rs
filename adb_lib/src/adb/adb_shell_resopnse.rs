use std::{
    io::{Read, Result},
    net::TcpStream,
};

pub struct ADBShellResponse {
    status: String,
    data: Vec<u8>,
}

impl std::fmt::Display for ADBShellResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status:{}, data: {:?}", self.status, self.data)
    }
}

impl std::fmt::Debug for ADBShellResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ADBShellResponse {{ status: {}, data: {:?} }}",
            self.status, self.data
        )
    }
}

impl ADBShellResponse {
    pub fn new(data: Vec<u8>) -> ADBShellResponse {
        let status = String::from_utf8_lossy(&[data[0], data[1], data[2], data[3]]).to_string();
        let data = data[4..].to_vec();

        ADBShellResponse { status, data }
    }

    pub fn with(mut stream: &TcpStream) -> Result<ADBShellResponse> {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;

        Ok(ADBShellResponse::new(buffer))
    }

    pub fn is_ok(&self) -> bool {
        self.status == "OKAY"
    }

    pub fn is_err(&self) -> bool {
        self.status != "OKAY"
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn data_to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

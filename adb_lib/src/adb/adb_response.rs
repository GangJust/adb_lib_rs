use std::{
    io::{Read, Result},
    net::TcpStream,
};

pub struct ADBResponse {
    status: String,
    length: u32,
    data: Vec<u8>,
}

impl std::fmt::Display for ADBResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "status: {}, length: {}, data: {:?}",
            self.status, self.length, self.data
        )
    }
}

impl std::fmt::Debug for ADBResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ADBResponse {{ status: {}, length: {}, data: {:?} }}",
            self.status, self.length, self.data
        )
    }
}

impl ADBResponse {
    ///
    #[allow(unused)]
    pub fn new(data: Vec<u8>) -> ADBResponse {
        let status = String::from_utf8_lossy(&[data[0], data[1], data[2], data[3]]).to_string();
        let length = String::from_utf8_lossy(&[data[4], data[5], data[6], data[7]]).to_string();
        let length = u32::from_str_radix(&length, 16).unwrap();
        let data = data[8..].to_vec();

        ADBResponse {
            status,
            length,
            data,
        }
    }

    ///
    pub fn with(mut stream: &TcpStream) -> Result<ADBResponse> {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;

        Ok(ADBResponse::new(buffer))
    }

    /// 判断响应是否成功
    #[allow(unused)]
    pub fn is_ok(&self) -> bool {
        self.status == "OKAY"
    }

    /// 判断响应是否失败
    #[allow(unused)]
    pub fn is_err(&self) -> bool {
        self.status != "OKAY"
    }

    /// 获取响应状态
    #[allow(unused)]
    pub fn status(&self) -> String {
        self.status.clone()
    }

    /// 获取响应长度
    #[allow(unused)]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// 获取响应数据
    #[allow(unused)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// 获取响应数据
    #[allow(unused)]
    pub fn data_to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

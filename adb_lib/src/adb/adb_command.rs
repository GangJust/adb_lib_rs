use core::str;

pub struct ADBCommand {
    cmd: String,
}

impl ADBCommand {
    pub fn new<T>(cmd: T) -> ADBCommand
    where
        T: AsRef<str>,
    {
        ADBCommand {
            cmd: cmd.as_ref().to_string(),
        }
    }

    /// get the raw command
    pub fn raw(&self) -> &str {
        &self.cmd
    }

    // format the command
    pub fn format(&self) -> String {
        let cmd_len = format!("{:04x}", self.cmd.len());
        format!("{}{}", cmd_len, self.cmd)
    }
}

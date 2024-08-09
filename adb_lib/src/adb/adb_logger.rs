pub struct ADBLogger;

impl ADBLogger {
    pub fn d<T>(msg: T)
    where
        T: AsRef<str> + std::fmt::Debug + std::fmt::Display,
    {
        if cfg!(debug_assertions) {
            println!("{}", msg);
        }
    }
}

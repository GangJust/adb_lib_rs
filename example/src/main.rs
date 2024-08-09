use adb_lib::{ADBConnection, ADBLogger};

fn main() {
    let conn = ADBConnection::new("localhost", 5037);

    println!("\n------------------ version ----------------------");
    let version = conn.version().expect("Failed to get ADB version");
    ADBLogger::d(version.data_to_string());

    println!("\n------------------ devices ----------------------");
    let devices = conn.devices().expect("Failed to get devices");
    ADBLogger::d(devices.data_to_string());

    println!("\n------------------ state ----------------------");
    let state = conn.state("").expect("Failed to get state");
    ADBLogger::d(state.data_to_string());

    println!("\n------------------ shell ----------------------");
    let shell = conn
        .shell("", "ls -lai /")
        .expect("Failed to select device");
    ADBLogger::d(shell.data_to_string());
}

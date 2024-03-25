use std::{*, time::Duration};
use serialport::*;



fn main() {
    println!("Hello, world!");

    let ports = serialport::available_ports().expect("No ports of use found");
    for p in ports {

        println!("{}",p.port_name);
    }

    let mut port = serialport::new("/dev/ttyACM0",115_200)
        .timeout(Duration::from_millis(100))
        .open().expect("Failed to Open the dang ol' port!");
   
    let output = "This is a test. This is only a test.".as_bytes();

   port.write(output).expect("Write failed!");


}

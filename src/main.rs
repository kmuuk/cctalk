use cctalk::emulator::CCTalkEmu;
use cctalk::protocol::{ChecksumType, Message};
use std::time::Duration;
fn main() {
    env_logger::init();
    let port = serialport::new("COM48", 9600).timeout(Duration::from_millis(5)).open().expect("Failed to open port");
    
    let test_device = CCTalkEmu::init(port);

    let mut t_dev = match test_device {
        Ok(dev) => dev,
        Err(error) => panic!("Panicked {:?}", error),
    };

    //let test_data: cctalk::protocol::Data = vec![];
    // let test_payload: cctalk:protocol::Payload =
    //let mut timestamp = Instant::now();
    loop {
        let mut test_msg: Vec<Message> = t_dev.read_messages();

        if test_msg.len() > 0 {
            //println!("Source: {}", test_msg[0].source);
            //println!("Destination: {}", test_msg[0].destination);
            //println!("Payload: {:?}", test_msg[0].payload);
            t_dev.reply_message(&test_msg.remove(0)).unwrap();
            //test_msg.clear();
        }
        //let now = Instant::now();
        //if timestamp.add(Duration::new(10,0)).lt(&now) {
            //t_dev.add_credit(1);
        //    timestamp = now;
        //}
        //thread::sleep(time::Duration::from_millis(10));
    }

    
}
use std::{net::SocketAddr, thread};

use xrp::{frc::motor::Motor, subsystem::Subsystem, subsystem::manager::SubsystemManager, network_tables::NetworkTable};
use nt::{EntryData, NetworkTables};
fn main() {
    let mut nt_table = NetworkTable::bind("127.0.0.1:5810");
    let mut a = Subsystem::new(Motor::new(0));
    let mut b = Subsystem::new(Motor::new(1));
    a.depends_on(&b);

    a.with_lock(|sub| sub.move_to(90));
    b.with_lock(|sub| sub.move_to(90));

    a.write().move_to(90);
    
    SubsystemManager::tracker().periodic_all();
    assert!(SubsystemManager::tracker().len() == 2);
    // let mut table = nt_table::server::BlockingServerHandle::start(SocketAddr, config)
    // let mut nt = NetworkTables::bind("127.0.0.1:5810", "nt-rs-server");
    // let a = pollster::block_on(nt.create_entry(EntryData::new("yoooooo".to_owned(), 0, nt::EntryValue::Boolean(false)))).unwrap();
    nt_table.publish_topic("aa", 1);
    

    loop {
        thread::sleep_ms(1);

        println!("{:?}", nt_table.get_topic_value(&"aa".to_owned()).unwrap());
    //     nt.update_entry(a, nt::EntryValue::Double(0.0));
    }
}

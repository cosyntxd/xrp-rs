use std::thread;

use xrp::{frc::motor::Motor, subsystem::Subsystem, subsystem::manager::SubsystemManager, network_tables::NetworkTable};
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

    loop {
        nt_table.publish_topic("amigo", 1).unwrap();
        nt_table.publish_topic("amigo", 1).unwrap();
        nt_table.set_topic_value("amigo", 1);
        nt_table.set_topic_value("amigo", 1);
        nt_table.set_topic_value_w_timestamp("amigo", 1, 0);
        println!("{:?}", nt_table.get_topic_value(&format!("amigo")));

        thread::sleep_ms(1);
    }
}

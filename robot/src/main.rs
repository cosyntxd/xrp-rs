use std::{net::SocketAddr, thread};

use xrp::{frc::physical::*, subsystem::Subsystem, subsystem::manager::SubsystemManager};
fn main() {
    let mut a = Subsystem::new(Motor::new(0));
    let mut b = Subsystem::new(Motor::new(1));
    a.depends_on(&b);

    a.with_lock(|sub| sub.set_power(1.0));
    b.with_lock(|sub| sub.set_power(1.0));

    a.write().set_power(1.0);
    
    SubsystemManager::tracker().periodic_all();
    assert!(SubsystemManager::tracker().len() == 2);

    loop {
        thread::sleep_ms(1);
    }
}
// that video with eveyrthig about chem/ physics

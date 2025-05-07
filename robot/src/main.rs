use xrp::{frc::motor::Motor, subsystem::Subsystem, subsystem::manager::SubsystemManager};
fn main() {
    let mut a = Subsystem::new(Motor::new());
    let mut b = Subsystem::new(Motor::new());
    a.depends_on(&b);
    SubsystemManager::tracker().periodic_all();
    assert!(SubsystemManager::tracker().len() == 2);
}


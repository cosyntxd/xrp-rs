use xrp::{frc::motor::Motor, subsystem::Subsystem, subsystem::manager::SubsystemManager};
fn main() {
    let mut a = Subsystem::new(Motor::new());
    let mut b = Subsystem::new(Motor::new());
    a.depends_on(&b);

    a.with_lock(|sub| sub.move_to(90));
    b.with_lock(|sub| sub.move_to(90));

    a.write().move_to(90);
    
    SubsystemManager::tracker().periodic_all();
    assert!(SubsystemManager::tracker().len() == 2);
}

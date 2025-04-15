use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

pub trait SubsystemTrait: Send + Sync {
    fn periodic(&mut self);
    fn on_packet(&mut self);
}

pub mod track_subsystem {
    use std::sync::{Arc, Mutex};

    use once_cell::sync::Lazy;

    use super::SubsystemTrait;

    static TRACKED_SUBSYSTEMS: Lazy<Mutex<Vec<Arc<Mutex<dyn SubsystemTrait>>>>> =
        Lazy::new(|| Mutex::new(Vec::new()));

    pub fn add_subsystem(subsystem: Arc<Mutex<dyn SubsystemTrait>>) {
        TRACKED_SUBSYSTEMS.lock().unwrap().push(subsystem);
    }
    pub fn execute_all_periodic(time: Option<u64>) {
        let subsystems = TRACKED_SUBSYSTEMS.lock().unwrap();
        for subsystem in subsystems.iter() {
            let mut guard = subsystem.lock().unwrap();
            guard.periodic();
        }
    }
    pub fn execute_all_recieve_packet(time: Option<u64>) {
        let subsystems = TRACKED_SUBSYSTEMS.lock().unwrap();
        for subsystem in subsystems.iter() {
            let mut guard = subsystem.lock().unwrap();
            guard.periodic();
        }
    }
}
pub struct Subsystem<T: SubsystemTrait + 'static> {
    inner: Arc<Mutex<T>>,
}

impl<T: SubsystemTrait + 'static> Subsystem<T> {
    pub fn new(subsystem: T) -> Self {
        let inner = Arc::new(Mutex::new(subsystem));
        track_subsystem::add_subsystem(inner.clone());
        Self { inner }
    }

    pub fn clone(&self) -> Subsystem<T> {
        Self { inner: self.inner.clone() }
    }

    pub fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().unwrap();
        f(&mut guard)
    }
}


pub fn subsystem<T>(subsystem: T) -> Subsystem<T>
where
    T: SubsystemTrait + 'static,
{
    Subsystem::new(subsystem)
}

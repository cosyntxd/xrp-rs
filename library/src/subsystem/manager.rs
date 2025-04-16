use std::collections::{HashSet, VecDeque};
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, Mutex, MutexGuard,
};

use once_cell::sync::Lazy;

use super::{Subsystem, SubsystemTrait};

static TRACKED_SUBSYSTEMS: Lazy<Arc<Mutex<Vec<Subsystem<dyn SubsystemTrait>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(vec![])));
static mut ID: u16 = 1;

pub fn get_subsystems() -> MutexGuard<'static, Vec<Subsystem<(dyn SubsystemTrait + 'static)>>> {
    TRACKED_SUBSYSTEMS.lock().unwrap()
}

pub fn add_subsystem(subsystem: Subsystem<dyn SubsystemTrait>) {
    get_subsystems().push(subsystem);
}

pub fn execute_all(mut func: impl FnMut(&mut dyn SubsystemTrait)) {
    let mut subsystems = get_subsystems();
    // todo: make better
    // few loc but worst case is O(n^2)
    for i in 0..500 {
        for sub in subsystems.iter_mut() {
            if (sub
                .get_mut()
                .guard
                .deps
                .iter_mut()
                .all(|v| v.get_mut().guard.execution_id == unsafe { ID }))
            {
                let a = &mut sub.get_mut().guard.inner.inner;
                if let Some(v) = a {
                    func(v.as_mut());
                }
            }
        }
    }
}

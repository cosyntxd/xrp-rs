// todo: is a mutex needed here

use once_cell::sync::Lazy;

use super::{StrongOpaque, Subsystem, SubsystemTrait, WeakOpaque};
static TRACKER: Lazy<SubsystemManager> = Lazy::new(|| SubsystemManager::new());
pub struct SubsystemManager {
    subsystems: Vec<WeakOpaque>,
}
impl SubsystemManager {
    pub fn new() -> Self {
        Self { subsystems: vec![] }
    }
    pub fn add_subsystem<T: SubsystemTrait>(&mut self, subsystem: Subsystem<T>) {
        self.subsystems.push(subsystem.as_opaque_weak());
    }
    pub fn get_subsystems(&mut self) -> &mut Vec<WeakOpaque> {
        &mut self.subsystems
    }
    pub fn get_subsystems_by_type<T: SubsystemTrait>(&mut self) -> Vec<StrongOpaque> {
        let mut out = vec![];
        for sub in self.get_subsystems() {
            if let Some(strong) = sub.upgrade() {
                let inner = &strong.read().unwrap().inner;
                if inner.inner.is::<T>() {
                    out.push(strong.clone())
                }
            }
        }
        out
    }
}
// use once_cell::sync::Lazy;

// // use super::{Subsystem, SubsystemTrait};

// static TRACKED_SUBSYSTEMS: Lazy<Arc<Mutex<Vec<Subsystem<dyn SubsystemTrait>>>>> =
//     Lazy::new(|| Arc::new(Mutex::new(vec![])));
// static mut EXECUTION_ID: u16 = 1;

// // pub fn get_subsystems() -> MutexGuard<'static, Vec<Subsystem<(dyn SubsystemTrait)>>> {
// //     TRACKED_SUBSYSTEMS.lock().unwrap()
// // }

// pub fn add_subsystem<T>(subsystem: Subsystem<T>)
// where
//     T: SubsystemTrait + Sized + 'static,
// {

//     // subsystem.as_dyn();
//     // get_subsystems().push(subsystem.clone().as_dyn());
// }

// // pub fn execute_all(mut func: impl FnMut(&mut dyn SubsystemTrait)) {
// //     let mut subsystems = get_subsystems();
// //     // todo: make better
// //     // few loc but worst case is O(n^2)
// //     // executes all subsystems while respecting dependencies
// //     // i dont wanna write it properly rn cuz that will be like +150 loc

// //     for i in 0..500 {
// //         for sub in subsystems.iter_mut() {
// //             if (sub
// //                 .get_mut()
// //                 .guard
// //                 .deps
// //                 .iter_mut()
// //                 .all(|v| v.get_mut().guard.execution_id == unsafe { ID }))
// //             {
// //                 let a = &mut sub.get_mut().guard.inner.inner;
// //                 if let Some(v) = a {
// //                     func(v.as_mut());
// //                 }
// //             }
// //         }
// //     }
// //     unsafe { ID += 1 };
// //     // doesnt respect deps but will run them all
// //     // for sub in subsystems.iter_mut() {
// //     //     let a = &mut sub.get_mut().guard.inner.inner;
// //     //     if let Some(v) = a {
// //     //         func(v.as_mut());
// //     //     }
// //     // }
// // }

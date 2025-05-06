// todo: is a mutex needed here
// todo: is an arc needed here
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use once_cell::sync::Lazy;

use super::{StrongOpaque, Subsystem, SubsystemManaged, SubsystemRaw, SubsystemTrait, WeakOpaque};
static TRACKER: Lazy<SubsystemManager> = Lazy::new(|| SubsystemManager::new());
pub struct SubsystemManager {
    subsystems: RwLock<Vec<WeakOpaque>>,
}
impl SubsystemManager {
    pub fn new() -> Self {
        Self {
            subsystems: RwLock::new(vec![]),
        }
    }
    pub fn get_subsystems(&mut self) -> RwLockWriteGuard<'_, Vec<WeakOpaque>> {
        self.subsystems.write().unwrap()
    }
    pub fn add_subsystem<T: SubsystemTrait>(&mut self, subsystem: Subsystem<T>) {
        self.get_subsystems().push(subsystem.as_opaque_weak());
    }
    pub fn get_subsystems_by_type<T: SubsystemTrait + 'static>(
        &mut self,
    ) -> impl Iterator<Item = StrongOpaque> {
        self.get_subsystems().clone().into_iter().filter_map(|sub| {
            sub.upgrade().and_then(|strong| {
                let inner = &strong.read().ok()?.inner;
                if inner.inner.is::<T>() {
                    Some(strong.clone())
                } else {
                    None
                }
            })
        })
    }

    pub fn execute_all_generic(&mut self, mut run: impl FnMut(&mut SubsystemRaw)) {
        self.get_subsystems().clone().retain(|weak| {
            if let Some(strong) = weak.upgrade() {
                if let Ok(mut guard) = strong.write() {
                    run(&mut guard.inner);
                }
                true
            } else {
                false // drop dead weak
            }
        });
    }
    pub fn periodic_all() {}
    pub fn write_packet_all() {}
    pub fn read_packet_all() {}
    pub fn remove_dropped(&mut self) {
        self.get_subsystems()
            .retain(|weak| weak.upgrade().is_some());
    }
    pub fn approximate_len(&mut self) -> usize {
        self.get_subsystems().len()
    }
    pub fn len(&mut self) -> usize {
        // self.subsystems.iter().count()
        0
    }
}

// todo: is a mutex needed here
// todo: is an arc needed here
use std::{ptr::addr_of_mut, sync::{RwLock, RwLockWriteGuard}, time::Instant};

use once_cell::sync::Lazy;

use super::{StrongOpaque, Subsystem, SubsystemRaw, SubsystemTrait, WeakOpaque};
static mut TRACKER: Lazy<SubsystemManager> = Lazy::new(|| SubsystemManager::new());
pub struct SubsystemManager {
    subsystems: RwLock<Vec<WeakOpaque>>,
}
impl SubsystemManager {
    pub fn new() -> Self {
        Self {
            subsystems: RwLock::new(vec![]),
        }
    }
    pub fn tracker() -> &'static mut Lazy<SubsystemManager> {
        unsafe { &mut (TRACKER) }
    }
    pub fn get_subsystems(&mut self) -> RwLockWriteGuard<'_, Vec<WeakOpaque>> {
        self.subsystems.write().unwrap()
    }
    pub fn add_opaque_subsystem(&mut self, subsystem: WeakOpaque) {
        self.get_subsystems().push(subsystem);
    }
    pub fn add_subsystem<T: SubsystemTrait>(&mut self, subsystem: Subsystem<T>) {
        self.add_opaque_subsystem(subsystem.as_opaque_weak());
    }
    
    pub fn get_subsystems_by_type<T: SubsystemTrait + 'static>(
        &mut self,
    ) -> impl Iterator<Item = StrongOpaque> {
        self.get_subsystems().clone().into_iter().filter_map(|sub| {
            sub.upgrade().and_then(|strong| {
                let inner = &strong.read().ok()?.inner;
                if inner.inner.as_any().is::<T>() {
                    Some(strong.clone())
                } else {
                    None
                }
            })
        })
    }

    pub fn execute_all_generic(&mut self, mut run: impl FnMut(&mut SubsystemRaw)) {
        self.get_subsystems().iter().for_each(|weak| {
            if let Some(strong) = weak.upgrade() {
                if let Ok(mut guard) = strong.write() {
                    run(&mut guard.inner);
                }
            }
        });
    }
    pub fn periodic_all(&mut self) {
        self.execute_all_generic(|sub| {
            let now = Instant::now();
            sub.inner.periodic();
            sub.last_periodic = now;
        });
    }
    pub fn read_packet_all(&mut self) {
        self.execute_all_generic(|sub| {
            let now = Instant::now();
            sub.inner.received_packet();
            sub.last_receive_packet = now;
        });
    }
    pub fn write_packet_all(&mut self) {
        self.execute_all_generic(|sub| {
            let now = Instant::now();
            sub.inner.sending_packet();
            sub.last_sent_packet = now;
        });
    }
    
    pub fn remove_dropped(&mut self) {
        self.get_subsystems()
            .retain(|weak| weak.upgrade().is_some());
    }
    pub fn approximate_len(&mut self) -> usize {
        self.get_subsystems().len()
    }

    pub fn len(&mut self) -> usize {
        self.get_subsystems().iter().filter(|weak| weak.upgrade().is_some()).count()
    }
}

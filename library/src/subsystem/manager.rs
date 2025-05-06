// todo: is a mutex needed here

use once_cell::sync::Lazy;

use super::{StrongOpaque, Subsystem, SubsystemRaw, SubsystemTrait, WeakOpaque};
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
    pub fn get_subsystems_by_type<T: SubsystemTrait + 'static>(
        &mut self,
    ) -> impl Iterator<Item = StrongOpaque> + '_ {
        self.get_subsystems()
            .into_iter()
            .filter_map(|sub| {
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
    pub fn execute_all_generic(&mut self, run: impl FnMut(&mut SubsystemRaw) -> ()) {
    }
    pub fn periodic_all() {

    }
    pub fn write_packet_all() {

    }
    pub fn read_packet_all() {

    }
}
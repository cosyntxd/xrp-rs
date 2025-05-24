use std::sync::Mutex;

use nt_rs::{instance::NetworkTableInstance, nt};
use once_cell::sync::Lazy;
// pub use nt_rs::ins
static INSTANCE: Lazy<Mutex<NetworkTableInstance>> = Lazy::new(|| {
    let mut instance = NetworkTableInstance::default();
    instance.start_server(Default::default());
    Mutex::new(instance)
});


#[macro_export]
macro_rules! nt_bind {
    (bind $addr:expr) => {{

    }}
}

#[macro_export]
macro_rules! nt_write {
    ($path:expr, $val:expr) => {{

    }};
}

#[macro_export]
macro_rules! nt_read {
    ($path:expr) => {{

    }};
}


pub fn is_bound() -> bool {
    unsafe { BOUND }
}

pub fn mark_bound() {
    unsafe {
        BOUND = true;
    }
}

pub fn bind_default() {
    let mut instance = get_instance();
    instance.start_server(Default::default());
    mark_bound();
}

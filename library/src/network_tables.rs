// use std::sync::Mutex;

// use nt_rs::{instance::NetworkTableInstance, nt};
// use once_cell::sync::Lazy;

// static INSTANCE: Lazy<Mutex<NetworkTableInstance>> = Lazy::new(|| {
//     let mut instance = NetworkTableInstance::default();
//     instance.start_server(Default::default());
//     Mutex::new(instance)
// });

// #[macro_export]
// macro_rules! nt_bind {
//     (bind $addr:expr) => {{
//         // INSTANCE.get_or_try_init()
//     }}
// }

// #[macro_export]
// macro_rules! nt_write {
//     ($path:expr, $val:expr) => {{

//     }};
// }

// #[macro_export]
// macro_rules! nt_read {
//     ($path:expr) => {{

//     }};
// }

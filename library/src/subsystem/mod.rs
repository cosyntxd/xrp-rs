pub mod manager;

use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicU16, Arc, Mutex, RwLock, RwLockWriteGuard},
    time::Instant,
};

pub trait SubsystemTrait: Send + Sync {
    fn periodic(&mut self);
    fn received_packet(&mut self);
    fn sending_packet(&mut self);
}
pub struct RawSubsystem<T: SubsystemTrait + ?Sized> {
    // optional just so i can drop it without informing every dependency who has arc ref
    // in theory it cann be None only when resolving dependencies and never for user
    pub inner: Option<Box<T>>,
    pub last_periodic: Instant,
}
struct SubsystemInner<T: SubsystemTrait + ?Sized> {
    pub inner: RawSubsystem<T>,
    pub deps: Vec<Subsystem<dyn SubsystemTrait>>,
    pub execution_id: u16,
}
pub struct Subsystem<T: SubsystemTrait + ?Sized> {
    pub inner: Arc<RwLock<SubsystemInner<T>>>,
}

impl<T: SubsystemTrait> Subsystem<T> {
    pub fn new(subsystem: T) -> Subsystem<T> {
        Self::from_box(Box::new(subsystem))
    }
}
impl<T: SubsystemTrait + ?Sized> Subsystem<T> {
    pub fn from_box(boxed_subsystem: Box<T>) -> Subsystem<T> {
        let managed_subsystem = Subsystem {
            inner: Arc::new(RwLock::new(SubsystemInner {
                inner: RawSubsystem {
                    inner: Some(boxed_subsystem),
                    last_periodic: Instant::now(),
                },
                deps: vec![],
                execution_id: 0,
            })),
        };
        managed_subsystem
    }
    pub fn depends_on(&mut self, other: &Subsystem<dyn SubsystemTrait>) {
        let mut inner = self.inner.write().unwrap();
        inner.deps.push(Subsystem {
            inner: other.inner.clone(),
        });
    }
    pub fn with_lock<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.write().expect("Failed to acquire write lock");
        f(&mut guard.inner.inner.as_mut().unwrap())
    }

    pub fn get_mut(&mut self) -> SubsystemGuard<T> {
        SubsystemGuard {
            guard: self.inner.write().ok().unwrap(),
        }
    }
}

impl<T: SubsystemTrait + ?Sized> Drop for Subsystem<T> {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.deps.clear();
            // todo: are both needed?
            if let Some(subsystem) = inner.inner.inner.take() {
                drop(subsystem);
            }
            inner.inner.inner = None;
        }
    }
}

pub struct SubsystemGuard<'a, T: SubsystemTrait + ?Sized> {
    guard: RwLockWriteGuard<'a, SubsystemInner<T>>,
}

impl<'a, T: SubsystemTrait + ?Sized> Deref for SubsystemGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.guard.inner.inner.as_ref().unwrap()
    }
}

impl<'a, T: SubsystemTrait + ?Sized> DerefMut for SubsystemGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guard.inner.inner.as_mut().unwrap()
    }
}

pub struct SubsystemReadGuard<'a, T: SubsystemTrait + ?Sized> {
    guard: std::sync::RwLockReadGuard<'a, SubsystemInner<T>>,
}

impl<'a, T: SubsystemTrait + ?Sized> Deref for SubsystemReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.guard.inner.inner.as_ref().unwrap()
    }
}

impl<T: SubsystemTrait + ?Sized> Subsystem<T> {
    pub fn read(&self) -> Option<SubsystemReadGuard<T>> {
        Some(SubsystemReadGuard {
            guard: self.inner.read().ok()?,
        })
    }
}

pub mod manager;

use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockWriteGuard},
    time::Instant,
};

pub trait SubsystemTrait: Send + Sync {
    // fn new(id: u8) -> Self;
    // fn new_sim();
    fn periodic(&mut self);
    fn received_packet(&mut self);
    fn sending_packet(&mut self);
}
pub struct RawSubsystem<T: SubsystemTrait + ?Sized> {
    // optional just so i can drop it without informing every dependency who has arc ref
    // in theory it cann be None only when resolving dependencies and never for user
    pub inner: Option<Box<T>>,
    pub creation: Instant,

    // should be Optional<Instant> but i dont think people coming from java will like
    // to do proper error handling. They are fine with errors just not the handling part
    pub last_periodic: Instant,
    pub last_sent_packet: Instant,
    pub last_recieve_packet: Instant,
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
        let time = Instant::now();
        let managed_subsystem = Subsystem {
            inner: Arc::new(RwLock::new(SubsystemInner {
                inner: RawSubsystem {
                    inner: Some(boxed_subsystem),
                    creation: time,
                    last_periodic: time,
                    last_sent_packet: time,
                    last_recieve_packet: time,
                },
                deps: vec![],
                execution_id: 0,
            })),
        };
        // manager::add_subsystem(&managed_subsystem);
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
            guard: self.inner.write().unwrap(),
        }
    }
    pub fn get_nonmut(&self) -> SubsystemReadGuard<T> {
        SubsystemReadGuard {
            guard: self.inner.read().unwrap(),
        }
    }
    // todo fancy impl decleration
    /// SAFETY: sould outlive original and if either is dropped, so will inner subsystem
    pub fn clone(&self) -> Subsystem<T> {
        Subsystem {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: SubsystemTrait + ?Sized> Drop for Subsystem<T> {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.write() {
            inner.deps.clear();
            // todo: are both needed?
            inner.inner.inner = None;
            if let Some(subsystem) = inner.inner.inner.take() {
                drop(subsystem);
            }
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

impl<T: SubsystemTrait + Sized + 'static> Clone for Subsystem<T> {
    fn clone(&self) -> Self {
        Subsystem {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: SubsystemTrait + ?Sized> PartialEq for Subsystem<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T: SubsystemTrait + ?Sized> Eq for Subsystem<T> {}

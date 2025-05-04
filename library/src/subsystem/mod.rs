pub mod manager;

use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
    time::Instant,
};

pub trait SubsystemTrait: Send + Sync + Any + 'static {
    fn periodic(&mut self);
    fn received_packet(&mut self);
    fn sending_packet(&mut self);
}
pub struct SubsystemRaw {
    pub inner: Box<dyn Any>,
    pub creation: Instant,
    pub last_periodic: Instant,
    pub last_sent_packet: Instant,
    pub last_receive_packet: Instant,
}
pub struct SubsystemManaged {
    pub inner: SubsystemRaw,
    pub deps: Vec<WeakOpaque>,
    pub execution_id: u16,
}
type StrongOpaque = Arc<RwLock<SubsystemManaged>>;
type WeakOpaque = Weak<RwLock<SubsystemManaged>>;

pub struct Subsystem<T: SubsystemTrait> {
    _type: PhantomData<T>,
    inner: StrongOpaque,
}

impl<T: SubsystemTrait> Subsystem<T> {
    pub fn new(subsystem: T) -> Subsystem<T> {
        let now = Instant::now();
        let raw = SubsystemRaw {
            inner: Box::new(subsystem),
            creation: now,
            last_periodic: now,
            last_sent_packet: now,
            last_receive_packet: now,
        };
        let managed = SubsystemManaged {
            inner: raw,
            deps: vec![],
            execution_id: 0,
        };
        let opaque = Arc::new(RwLock::new(managed));
        Subsystem {
            _type: PhantomData,
            inner: opaque,
        }
    }
    pub fn depends_on<S: SubsystemTrait>(&mut self, other: &Subsystem<S>) {
        let mut managed = self.inner.write().unwrap();
        managed.deps.push(other.clone().as_opaque_weak());
    }
    pub fn with_lock<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        f(&mut self.write())
    }

    pub fn read(&self) -> SubsystemReadGuard<T> {
        let guard = self.inner.read().unwrap();
        SubsystemReadGuard {
            _type: PhantomData,
            guard,
        }
    }
    pub fn write(&mut self) -> SubsystemWriteGuard<T> {
        let guard = self.inner.write().unwrap();
        SubsystemWriteGuard {
            _type: PhantomData,
            guard,
        }
    }
    // This can be very dangerous
    pub unsafe fn cast<S: SubsystemTrait>(self) -> Subsystem<S> {
        todo!()
    }
    pub fn as_opaque_strong(self) -> StrongOpaque {
        self.inner
    }
    pub fn as_opaque_weak(self) -> WeakOpaque {
        Arc::<RwLock<SubsystemManaged>>::downgrade(&self.inner)
    }
}

impl<T: SubsystemTrait + Sized + 'static> Clone for Subsystem<T> {
    fn clone(&self) -> Self {
        Subsystem {
            _type: self._type,
            inner: self.inner.clone(),
        }
    }
}

impl<T: SubsystemTrait> PartialEq for Subsystem<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T: SubsystemTrait> Eq for Subsystem<T> {}

pub struct SubsystemReadGuard<'a, T: SubsystemTrait> {
    guard: RwLockReadGuard<'a, SubsystemManaged>,
    _type: PhantomData<T>,
}

impl<'a, T: SubsystemTrait> Deref for SubsystemReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.inner.inner.downcast_ref::<T>().unwrap()
    }
}

pub struct SubsystemWriteGuard<'a, T: SubsystemTrait> {
    guard: RwLockWriteGuard<'a, SubsystemManaged>,
    _type: PhantomData<T>,
}

impl<'a, T: SubsystemTrait> Deref for SubsystemWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.inner.inner.downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: SubsystemTrait> DerefMut for SubsystemWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.inner.inner.downcast_mut::<T>().unwrap()
    }
}

pub mod manager;

use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
    time::Instant,
};

use manager::SubsystemManager;

use crate::network::{recieve::XRPReceivePacket, send::XRPSendPacket};

pub trait SubsystemTrait: AsAny + Any + Send + Sync + 'static {
    fn periodic(&mut self) {}
    fn received_packet(&mut self, packet: &XRPReceivePacket) {}
    fn sending_packet(&mut self, packet: &mut XRPSendPacket) {}
}

impl<T: SubsystemTrait> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct SubsystemRaw {
    pub inner: Box<dyn SubsystemTrait + Send + Sync>,
    pub creation: Instant,
    pub last_periodic: Instant,
    pub last_sent_packet: Instant,
    pub last_receive_packet: Instant,
}
/// SAFETY: inner is guaranteed to implement SubsystemTrait
// todo: check with miri for soundness
impl SubsystemRaw {
    pub fn as_ref<T: SubsystemTrait>(&self) -> &T {
        self.inner.as_any().downcast_ref::<T>().unwrap()
    }
    pub fn as_mut<T: SubsystemTrait>(&mut self) -> &mut T {
        self.inner.as_any_mut().downcast_mut::<T>().unwrap()
    }
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
        SubsystemManager::tracker().add_opaque_subsystem(Arc::downgrade(&opaque.clone()));
        Subsystem {
            _type: PhantomData,
            inner: opaque,
        }
    }
    pub fn depends_on<S: SubsystemTrait>(&mut self, other: &Subsystem<S>) {
        self.write().guard.deps.push(other.clone().as_opaque_weak());
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
    /// Safety: T and S both implement SubsystemTrait but no guarantees they can cast to eachother.
    /// Casting is deferred for later and user should implement converting inner subsystem from T to S
    pub unsafe fn cast<S: SubsystemTrait>(self) -> Subsystem<S> {
        Subsystem {
            _type: PhantomData,
            inner: self.inner,
        }
    }
    pub fn as_opaque_strong(self) -> StrongOpaque {
        self.inner
    }
    pub fn as_opaque_weak(self) -> WeakOpaque {
        StrongOpaque::downgrade(&self.inner)
    }
}

impl<T: SubsystemTrait + 'static> Clone for Subsystem<T> {
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
// verify if: downcasting can be unchecked and unwraps can never fail
// unchecked downcasting is 4 instructions on x86 but unchecked unwrap is 11
// user controlls type of Any, so currently no guarantees that it can cast

impl<T: SubsystemTrait> Eq for Subsystem<T> {}

pub struct SubsystemReadGuard<'a, T: SubsystemTrait> {
    guard: RwLockReadGuard<'a, SubsystemManaged>,
    _type: PhantomData<T>,
}

impl<'a, T: SubsystemTrait> Deref for SubsystemReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.inner.inner.as_any().downcast_ref::<T>().unwrap()
    }
}

pub struct SubsystemWriteGuard<'a, T: SubsystemTrait> {
    guard: RwLockWriteGuard<'a, SubsystemManaged>,
    _type: PhantomData<T>,
}

impl<'a, T: SubsystemTrait> Deref for SubsystemWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.inner.inner.as_any().downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: SubsystemTrait> DerefMut for SubsystemWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.inner.inner.as_any_mut().downcast_mut::<T>().unwrap()
    }
}

flat_mod! { point, ambient }

use std::sync::Arc;
use crate::{math::Vec3};

pub type DynLight<'a> = Box<dyn 'a + Light>;

pub trait Light: Send + Sync {
    fn hits (&self, point: Vec3) -> Option<Vec3>;
}

impl<T: ?Sized + Light> Light for &T {
    #[inline]
    fn hits (&self, point: Vec3) -> Option<Vec3> {
        T::hits(*self, point)
    }
}

impl<T: ?Sized + Light> Light for Box<T> {
    #[inline]
    fn hits (&self, point: Vec3) -> Option<Vec3> {
        T::hits(self, point)
    }
}

impl<T: ?Sized + Light> Light for Arc<T> {
    #[inline]
    fn hits (&self, point: Vec3) -> Option<Vec3> {
        T::hits(self, point)
    }
}
use std::sync::{Arc, Weak};

trait Drawable {

}


pub struct Element<T: Drawable> {
    inner: Arc<T>,
}
pub struct WeakElement<T: Drawable> {
    inner: Weak<T>,

}
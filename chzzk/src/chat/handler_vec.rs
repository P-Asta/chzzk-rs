/// Closures returning futures:
/// They are often not Fn, neither FnMut. Because they can't reference capture
/// local environment, since they are async and may live longer than the
/// environment. All enviroments necessary should be moved.

/// Calling FnOnce moves the closure, so it can't be called again. But we do
/// not want this. So we need to clone the closure before calling it.

/// But traits with Clone cannot be trait objects since it is sized. So we
/// can't use Vec<Box<dyn FnOnce<T> + Clone>> directly.
use std::{future::Future, pin::Pin, sync::Arc};

pub(super) trait Handler<T>: Send + Sync {
    fn call(&self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<F, Fut, T> Handler<T> for F
where
    F: FnOnce(Arc<T>) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = ()> + Send + 'static,
    T: Send,
{
    fn call(&self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(self.clone()(v))
    }
}

pub(super) struct HandlerVec<T>(pub Vec<Box<dyn Handler<T>>>);

impl<T> HandlerVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub async fn broadcast(&self, payload: T) {
        let arc = Arc::new(payload);
        for i in &self.0 {
            i.call(arc.clone()).await;
        }
    }
}

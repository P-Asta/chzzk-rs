use std::{future::Future, pin::Pin, sync::Arc};

pub(crate) trait Handler<T>: Send + Clone {
    fn call(self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<F, Fut, T> Handler<T> for F
where
    F: FnOnce(Arc<T>) -> Fut + Clone + Send,
    Fut: Future<Output = ()> + Send + 'static,
    T: Send,
{
    fn call(self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(self(v))
    }
}

pub(super) struct HandlerHolder<H> {
    pub handler: H,
}

pub(super) trait HandlerTrait<T>: Send + Sync {
    fn call(&self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<H, T> HandlerTrait<T> for HandlerHolder<H>
where
    H: Handler<T> + Clone + Send + Sync,
    T: Send,
{
    fn call(&self, v: Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let h = self.handler.clone();
        h.call(v)
    }
}

pub(super) struct HandlerVec<T>(pub Vec<Box<dyn HandlerTrait<T>>>);

impl<T: Clone> HandlerVec<T> {
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

use std::{future::Future, pin::Pin};



pub(crate) trait Handler<T>: Send + Clone + 'static {
    fn call(self, v: T) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<F, Fut, T> Handler<T> for F
where
    F: FnOnce(T) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    T: Send + 'static,
{
    fn call(self, v: T) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(self(v))
    }
}

pub(super) struct HandlerHolder<H> {
    pub handler: H,
}

pub(super) trait HandlerTrait<T>: Send + Sync {
    fn call(&self, v: T) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<H, T> HandlerTrait<T> for HandlerHolder<H>
where
    H: Handler<T> + Clone + Send + Sync + 'static,
    T: Send,
{
    fn call(&self, v: T) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let h = self.handler.clone();
        h.call(v)
    }
}

pub(super) struct HandlerVec<T>(pub Vec<Box<dyn HandlerTrait<T>>>);

impl<T: Clone> HandlerVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub async fn broadcast(&mut self, payload: T) {
        for i in &self.0 {
            i.call(payload.clone()).await;
        }
    }
}

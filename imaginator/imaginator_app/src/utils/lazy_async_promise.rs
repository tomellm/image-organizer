use lazy_async_promise::{
    BoxedSendError, DirectCacheAccess, ImmediateValuePromise, ImmediateValueState,
};

pub trait ChainLazyAsyncPromise<U: Send> {
    fn then<F, R>(self, function: F) -> ImmediateValuePromise<U>
    where
        R: std::future::Future<Output = Result<U, BoxedSendError>> + Send + 'static,
        F: (FnOnce() -> R) + Send + 'static;
}

impl<T, U> ChainLazyAsyncPromise<U> for ImmediateValuePromise<T>
where
    T: Send + 'static,
    U: Send + 'static,
{
    fn then<F, R>(mut self, function: F) -> ImmediateValuePromise<U>
    where
        R: std::future::Future<Output = Result<U, BoxedSendError>> + Send + 'static,
        F: FnOnce() -> R + Send + 'static,
    {
        ImmediateValuePromise::new(async move {
            loop {
                match self.poll_state() {
                    ImmediateValueState::Updating => continue,
                    ImmediateValueState::Empty => unreachable!(),
                    _ => break,
                }
            }
            let Some(first_result) = self.take_result() else {
                unreachable!()
            };
            let _ = first_result?;
            function().await
        })
    }
}

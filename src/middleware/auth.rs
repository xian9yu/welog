use axum::{
    body::{Body, BoxBody},
    http::{Request, Response},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    pub(crate) inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("`AuthMiddleware` called!");

        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response<BoxBody> = inner.call(req).await?;

            println!("`AuthMiddleware` received the response");

            Ok(res)
        })
    }
}

use axum::extract::Request;
use axum::response::Response;
use futures_util::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone, Debug)]
pub struct CacheControlLayer {
    lifespan: u64,
}

impl CacheControlLayer {
    pub const fn with_lifespan(lifespan: u64) -> Self {
        CacheControlLayer { lifespan }
    }
}

impl<S> Layer<S> for CacheControlLayer {
    type Service = CacheControlMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheControlMiddleware {
            inner,
            lifespan: self.lifespan,
        }
    }
}

#[derive(Clone)]
pub struct CacheControlMiddleware<S> {
    inner: S,
    lifespan: u64,
}

impl<S> Service<Request> for CacheControlMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let future = self.inner.call(req);
        let cache_control = format!("public, max-age={}", self.lifespan);

        Box::pin(async move {
            let mut response = future.await?;
            response.headers_mut().insert(
                axum::http::header::CACHE_CONTROL,
                axum::http::HeaderValue::from_str(&cache_control).unwrap(),
            );
            Ok(response)
        })
    }
}

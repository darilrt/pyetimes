use std::str::FromStr;

use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
};

#[derive(Clone, Copy, Debug)]
pub struct StaticCached<T, const A: usize>(pub T);

impl<T, const A: usize> IntoResponse for StaticCached<T, A>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut response = self.0.into_response();
        response.headers_mut().insert(
            "Cache-Control",
            HeaderValue::from_str(&format!("public, max-age={}", A)).expect("No deberia de ocurrir"),
        );
        response
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CacheConfig<T> {
    pub value: T,
    pub max_age: usize,
    pub public: bool,
    pub must_revalidate: bool,
    pub proxy_revalidate: bool,
    pub no_store: bool,
    pub no_cache: bool,
}

impl<T: IntoResponse> IntoResponse for CacheConfig<T> {
    fn into_response(self) -> Response {
        let mut cache_control: Vec<String> = Vec::with_capacity(6);
        if self.no_store { 
            cache_control.push("no-store".to_string()); 
        }
        if self.no_cache { 
            cache_control.push("no-cache".to_string()); 
        }
        if self.max_age > 0 {
            cache_control.push(format!("max-age={}", self.max_age));
        }
        if self.must_revalidate {
            cache_control.push("must-revalidate".to_string());
        }
        if self.proxy_revalidate {
            cache_control.push("proxy-revalidate".to_string());
        }

        let mut response = self.value.into_response();
        let header_value =  HeaderValue::from_str(&cache_control.join(", ")).expect("Must join the cache controls keys");

        response.headers_mut().insert("Cache-Control",header_value,);
        response
    }
}

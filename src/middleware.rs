use std::{
    borrow::Cow,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use clap::{Parser, Subcommand};
use futures::{channel::oneshot, future, FutureExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use tower::Service;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use zksync_web3_decl::jsonrpsee::{
    core::async_trait,
    server::{ServerBuilder, TowerServiceBuilder},
};
use zksync_web3_decl::{jsonrpsee::RpcModule, namespaces::EthNamespaceServer};

use crate::proxy::Proxy;

use base64::decode;
use futures::future::BoxFuture;
use hyper::{Body, Request, Response};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Layer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use zksync_web3_decl::jsonrpsee::http_client::types::Request as JsonRpcRequest;
//use zksync_web3_decl::jsonrpsee::http_server::{
//    HttpServerBuilder, HttpServerMiddleware, Request as JsonRpcRequest, Response as JsonRpcResponse,
//};
use zksync_web3_decl::jsonrpsee::proc_macros::rpc;
use zksync_web3_decl::jsonrpsee::types::Params;

// Custom middleware to intercept and modify requests
pub struct AuthMiddleware<S> {
    inner: S,
}

#[derive(Clone, Debug, Serialize)]
struct AuthInfo {
    username: String,
    password: String,
}

use once_cell::sync::Lazy;
use std::collections::HashMap;

static REQUEST_AUTH_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("eth_getBalance", "privateeth_getBalance");
    m.insert("eth_blockNumber", "privateeth_blockNumber");
    m.insert("three", "cc");
    m
});

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + Clone + 'static,
    S::Future: Send,
{
    type Response = Response<Body>;
    type Error = S::Error; //hyper::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Basic ") {
                        let encoded_credentials = &auth_str[6..];
                        if let Ok(decoded_credentials) = decode(encoded_credentials) {
                            if let Ok(credentials) = String::from_utf8(decoded_credentials) {
                                let mut parts = credentials.splitn(2, ':');
                                if let (Some(username), Some(password)) =
                                    (parts.next(), parts.next())
                                {
                                    // Intercept and modify JSON-RPC requests
                                    let (parts, body) = req.into_parts();
                                    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
                                    let mut json_rpc_request: JsonRpcRequest =
                                        serde_json::from_slice(&body_bytes).unwrap();

                                    if let Some(private_method) =
                                        REQUEST_AUTH_MAP.get(json_rpc_request.method_name())
                                    {
                                        json_rpc_request.method = (*private_method).into();

                                        let mut params_json =
                                            if let Some(prev) = json_rpc_request.params {
                                                let value: Value =
                                                    serde_json::from_str(prev.get()).unwrap();

                                                value
                                            } else {
                                                Value::Array(vec![])
                                            };

                                        if let Some(arr) = params_json.as_array_mut() {
                                            arr.insert(0, json!(password));
                                            arr.insert(0, json!(username));
                                        }

                                        let modified_json_string =
                                            serde_json::to_string(&params_json).unwrap();

                                        let new_raw_value =
                                            RawValue::from_string(modified_json_string).unwrap();

                                        json_rpc_request.params = Some(Cow::Owned(new_raw_value));
                                    }

                                    let modified_body =
                                        serde_json::to_vec(&json_rpc_request).unwrap();
                                    let modified_req =
                                        Request::from_parts(parts, Body::from(modified_body));

                                    return inner.call(modified_req).await;
                                }
                            }
                        }
                    }
                }
            }

            inner.call(req).await
        })
    }
}

// Layer implementation for AuthMiddleware
pub struct AuthMiddlewareLayer;

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddleware { inner: service }
    }
}

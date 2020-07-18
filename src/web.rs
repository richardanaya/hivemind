use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn hello(_: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new(hyper::Body::from("Hello World!")))
}

pub async fn start_web_server() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr).serve(make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello))
    }));
    server.await.unwrap();
}

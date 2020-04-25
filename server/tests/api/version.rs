use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use crate::common::{
    default_application_accept_header, get_request_endpoint_string, routes_under_test,
};

#[tokio::test]
async fn accepts_accept_header_with_valid_api_version() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header(
            "accept",
            format!("{}+text", default_application_accept_header()),
        )
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn refuses_accept_header_with_invalid_api_version() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "application/vnd.warpj.vinvalid+text")
        .reply(&api)
        .await;

    asserting("returns NOT_ACCEPTABLE status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("body describes api version validation error")
        .that(&body)
        .contains("\"message\":\"could not find an api version in accept header\"")
}

#[tokio::test]
async fn refuses_accept_header_with_incorrect_api_version() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "application/vnd.warpj.v2500+text")
        .reply(&api)
        .await;

    asserting("returns NOT_ACCEPTABLE status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("body describes api version validation error")
        .that(&body)
        .contains("message\":\"api version 2500 is incorrect\"")
}

#[tokio::test]
async fn validation_is_skipped_if_accept_header_is_not_present() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn validation_is_skipped_if_client_accepts_any_content_type() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "*/*")
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

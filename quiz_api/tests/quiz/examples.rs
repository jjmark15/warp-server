use crate::common::web::requests::get_request_url;
use crate::common::{state::TestState, web::Endpoint};
use http::StatusCode;
use serde::Deserialize;
use spectral::prelude::*;

#[tokio::test]
async fn gives_example_question_set() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(
        state.server_http_address(),
        Endpoint::ExampleQuizQuestionSet,
    );

    state.request_builder().with_url(url).send().await.unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);

    asserting("json response body is an example question set")
        .that(&resp.json().await.unwrap())
        .is_equal_to(QuestionSet {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
            name: "Example question set title".to_string(),
        });
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct QuestionSet {
    id: String,
    name: String,
}

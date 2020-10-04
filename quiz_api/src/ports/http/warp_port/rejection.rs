use log::debug;
use warp::Rejection;

use crate::ports::http::warp_port::filters::validate::api::error::ApiValidationError;
use crate::ports::http::warp_port::response::{ErrorResponseMapper, WebErrorResponse};
use crate::ports::logging::log_string;
use warp::http::StatusCode;

pub(crate) async fn handle_rejection(rej: Rejection) -> Result<impl warp::reply::Reply, Rejection> {
    let error_response_mapper = ErrorResponseMapper::new();
    if let Some(err) = rej.find::<ApiValidationError>() {
        debug!("{}", log_string(err));
        Ok(error_response_mapper.map(err))
    } else {
        Err(rej)
    }
}

impl WebErrorResponse for ApiValidationError {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
}

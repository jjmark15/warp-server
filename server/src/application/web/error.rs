use warp::Reply;

use crate::application::logging::{LogEntry, LogEntryKVP};
use crate::application::web::response::{ErrorMessage, ErrorResponse};

impl LogEntry for ErrorMessage {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "rejection"),
            LogEntryKVP::new("code", format!("{}", self.code())),
            LogEntryKVP::new("message", self.message()),
        ]
    }
}

pub(crate) trait WebErrorResponse: std::error::Error {
    fn http_status_code(&self) -> warp::http::StatusCode;

    fn error_message(&self) -> ErrorMessage {
        ErrorMessage::new(self.http_status_code().as_u16(), format!("{}", self))
    }

    fn error_response(&self) -> ErrorResponse {
        ErrorResponse(
            warp::reply::with_status::<warp::reply::Json>(
                self.error_message().into(),
                self.http_status_code(),
            )
            .into_response(),
        )
    }
}

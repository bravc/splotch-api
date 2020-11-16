use actix_web::http::StatusCode;
use actix_web::{dev::HttpResponseBuilder, http::header, HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum SplotchError {
    #[display(fmt = "internal error")]
    InternalError,

//     #[display(fmt = "bad request")]
//     BadClientData,

//     #[display(fmt = "timeout")]
//     Timeout,
}

impl ResponseError for SplotchError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            SplotchError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            // SplotchError::BadClientData => StatusCode::BAD_REQUEST,
            // SplotchError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl From<reqwest::Error> for SplotchError {
    fn from(e: reqwest::Error) -> Self {
        log::error!("{}", e);
        SplotchError::InternalError
    }
}

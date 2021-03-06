use std::io::Cursor;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response, ResponseBuilder};

#[derive(Debug)]
pub enum ServerError {
    // Internal errors
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    PostgresError(postgres::Error),
    DateParseError(chrono::format::ParseError),

    // Web facing
    InvalidPlatform,
    InvalidChannel,
    StatusCode(rocket::http::Status),
}

impl Responder<'_> for ServerError {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let mut resp = Response::build();

        match self {
            Self::InvalidPlatform => {
                resp.status(Status::BadRequest);
                resp.sized_body(Cursor::new(format!(
                    "Invalid platform. Currently supported are windows and linux."
                ))); // TODO: Do not hardcode
            }
            Self::InvalidChannel => {
                resp.status(Status::BadRequest);
                resp.sized_body(Cursor::new(format!(
                    "Invalid channel. Currently supported is nightly with upcoming support for releases."
                ))); // TODO: Do not hardcode
            }
            Self::StatusCode(x) => {
                resp.status(x);
            }

            x => internal(&mut resp, x),
        }

        Ok(resp.finalize())
    }
}

fn internal<'r, T: std::fmt::Debug>(resp: &mut ResponseBuilder<'r>, error: T) {
    resp.status(Status::InternalServerError);
    log::error!("Internal Error: {:?}", error);
}

impl From<postgres::Error> for ServerError {
    fn from(error: postgres::Error) -> Self {
        Self::PostgresError(error)
    }
}

impl From<rocket::http::Status> for ServerError {
    fn from(error: rocket::http::Status) -> Self {
        Self::StatusCode(error)
    }
}

impl From<chrono::format::ParseError> for ServerError {
    fn from(error: chrono::format::ParseError) -> Self {
        Self::DateParseError(error)
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

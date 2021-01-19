extern crate reqwest;
extern crate serde_json;

use std::io;

#[derive(Debug)]
pub enum Error {
    // Failed to make post/get request or Failed to get text
    ReqwestError(reqwest::Error),
    // Parsing response failed
    JsonParsingError (serde_json::Error),
    // Failed to read data file
    FileReadingError(io::Error),
    // An empty image was provided.
    EmptyImage,
    // Provided token is invalid.
    InvalidToken,
    // Provided entity's size was greater than 10mb.
    EntityTooLarge,
    // API Limit is reached or Too many requests were made in short period of time
    TooManyRequests,
    // Something wrong with the trace.moe server or Image provided was malformed
    ServerError,
    // Status code given was not expected
    InvalidStatusCode
}

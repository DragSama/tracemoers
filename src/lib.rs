extern crate reqwest;
extern crate serde_json;

pub mod types;
pub mod errors;

use types::{Me, DataType, Search};
use errors::Error;
use std::{fs::{read, File}, result, path::Path};

pub struct Client {
    pub baseurl: String,
    pub mediaurl: String,
    pub token: Option<String>,
    pub req_client: reqwest::Client
}

pub type Result<T> = result::Result<T, Error>;

impl Client {
    pub fn new() -> Self {
        Self {
            baseurl: String::from("https://trace.moe/api"),
            mediaurl: String::from("https://media.trace.moe/"),
            token: None,
            req_client: reqwest::Client::new()
        }
    }

    pub fn set_token(&mut self, token: &str) -> &mut Client {
        self.token = Some(token.to_string());
        self
    }

    pub async fn get_me(&mut self) -> Result <Me>{
        let url = if self.token.is_some(){
            format!("{}{}{}", self.baseurl, "/me?token=", self.token.as_ref().unwrap())
        } else {
            format!("{}{}", self.baseurl, "/me")
        };
        let resp = match self.req_client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(Error::ReqwestError(e))
        };
        match serde_json::from_str::<Me>(&resp.text().await.unwrap()){
            Ok(json) => return Ok(json),
            Err(e) => return Err(Error::JsonParsingError(e))
        };
    }
    pub async fn save(&mut self, preview_type: PreviewType, path: &str) -> Result<()>{
        let url = match preview_type {
            PreviewType::Natural => format!("{}video/{}/{}?t={}&token={}", self.mediaurl, self.anilist_id, self.filename, self.at, self.tokenthumb),
            PreviewType::Image => format!("{}/thumbnail.php?anilist_id={}&file={}&t={}&token={}", self.baseurl, self.anilist_id, self.filename, self.at, self.tokenthumb)
        }
        let path = Path::new(path);
        let file = File::create(&path);
        let bytes = match self.req_client.get(&url).send().await {
            Ok(resp) => resp::bytes,
            Err(e) => return Err(Error::ReqwestError(e))
        };
        file.write_all(bytes);
        Ok(())
    }
    pub async fn search(&mut self, data: DataType) -> Result<Search> {
        let req_url = if self.token.is_some(){
            format!("{}{}{}", self.baseurl, "/search?token=", self.token.as_ref().unwrap())
        } else {
            format!("{}{}", self.baseurl, "/search")
        };
        let resp = match data {
            DataType::URL(url) => {
                match self.req_client.get(&format!("{}?url={}", req_url, url)).send().await {
                    Ok(resp) => resp,
                    Err(e) => return Err(Error::ReqwestError(e))
                }
            },
            DataType::File(file) => {
                let content = match read(file) {
                    Ok(r) => r,
                    Err(e) => return Err(Error::FileReadingError(e))
                };
                let part = reqwest::multipart::Part::bytes(content).file_name("image");
                let form = reqwest::multipart::Form::new().part("image", part);

                match self.req_client.post(&req_url).multipart(form).send().await {
                    Ok(resp) => resp,
                    Err(e) => return Err(Error::ReqwestError(e))
                }
            }
        };
        match resp.status().as_u16() {
            200 => {
                let text = match resp.text().await{
                    Ok(t) => t,
                    Err(e) => return Err(Error::ReqwestError(e))
                };
                match serde_json::from_str::<Search>(&text){
                        Ok(json) => return Ok(json),
                        Err(e) => return Err(Error::JsonParsingError(e))
                }
            },
            400 => return Err(Error::EmptyImage),
            403 => return Err(Error::InvalidToken),
            413 => return Err(Error::EntityTooLarge),
            429 => return Err(Error::TooManyRequests),
            500 | 503 => return Err(Error::ServerError),
            _ => return Err(Error::InvalidStatusCode)
        };
    }
}

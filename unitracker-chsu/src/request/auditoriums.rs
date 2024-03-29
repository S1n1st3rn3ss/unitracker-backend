use reqwest::{ClientBuilder, Method, StatusCode};
use crate::model::auditoriums::*;
use crate::request::RequestErrors;
use crate::request::constants::*;

pub async fn get_auditoriums(bearer: &str) -> Result<AuditoriumList, RequestErrors> {
    let auditorium_url = BASE_URL.to_owned() + AUDITORIUM;
    let client = ClientBuilder::new().user_agent("").build()?;
    let mut response = client
        .request(Method::GET, auditorium_url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send()
        .await?;
    let response_json = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    };
    response_json
}
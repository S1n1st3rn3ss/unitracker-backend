use reqwest::{Client, ClientBuilder, Method, StatusCode};
use crate::model::auditoriums::*;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};

pub async fn get_auditoriums(client: &mut Client, bearer: &str) -> Result<AuditoriumList, RequestErrors> {
    let auditorium_url = BASE_URL.to_owned() + AUDITORIUM;
    let response = call_with_url(client, &auditorium_url, bearer).await?;
    check_result(response).await
}
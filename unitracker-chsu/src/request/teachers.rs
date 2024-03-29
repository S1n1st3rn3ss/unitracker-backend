use reqwest::{ClientBuilder, Method, StatusCode};
use crate::model::teachers::TeacherList;
use reqwest::Error as ReqwestErrorType;
use crate::request::RequestErrors;
use crate::request::constants::*;

/// Get a [`list of all Teachers`](crate::model::teachers::TeacherList) in the university
pub async fn get_teachers(bearer: &str) -> Result<TeacherList, RequestErrors> {
    let teachers_url = BASE_URL.to_owned() + TEACHERS;
    let client = ClientBuilder::new().user_agent("").build()?;
    let mut response = client
        .request(Method::GET, teachers_url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send()
        .await?;
    let response_result = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    };
    response_result
}
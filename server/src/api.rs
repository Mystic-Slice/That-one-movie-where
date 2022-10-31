use std::{fmt, str::FromStr, fs};
use axum::{response::{ IntoResponse, Response}, extract::Query, body};
use serde::{de, Deserialize, Deserializer};
use movie_types::movie::Movie;

#[derive(Debug, Deserialize)]
/// Parameters in the http request
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    movie_id: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn build_error_response(msg: String) -> Response {
    (
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        msg
    ).into_response()
}

pub async fn get_movie_info(Query(params): Query<Params>) -> Response {
    let movie_id = params.movie_id.unwrap();

    const API_URL: &str = "http://www.omdbapi.com/";
    let api_key = fs::read_to_string("not_api_key.txt").unwrap();

    let client = reqwest::Client::new();

    let response = client.get(API_URL)
        .query(&[("apikey", api_key.as_str()), ("i", &movie_id), ("plot", "full")])
        .send().await;
    
    if let Err(error) = response {
        return build_error_response(format!("Error in API response: {error}").to_string())
    }

    let response = response.unwrap().text().await;

    if let Err(error) = response {
        return build_error_response(format!("Could not convert API response to text: {error}").to_string())
    }
    
    let json_response = json::parse(&response.unwrap()).unwrap();

    if json_response["Response"] == "False" {
        return build_error_response(format!("Movie with id {movie_id} not found").to_string())
    }

    // Check if it is a movie
    if json_response["Type"].to_string() != "movie" {
        return build_error_response(format!("{} is not a movie", json_response["Title"].to_string()).to_string())
    }

    // Check if all the required information is present
    let req_keys = vec!["Title", "imdbRating", "Poster", "Type"];
    for key in req_keys {
        if !json_response.has_key(key) {
            return build_error_response(format!("Could not get the complete info about the movie with id {movie_id}").to_string())
        } else if json_response[key] == "N/A" {
            return build_error_response(format!("Could not get the complete info about the movie with id {movie_id}").to_string())
        }
    }

    Response::new(
        body::boxed(
            serde_json::to_string(                
                &Movie::new(
                    json_response["Title"].to_string(),
                    json_response["imdbRating"].to_string().parse().unwrap(),
                    json_response["Poster"].to_string(),
                )
            ).unwrap()
        )
    )
}

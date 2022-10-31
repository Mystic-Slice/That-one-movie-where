use movie_types::{movie::Movie, movie_error::MovieError};
use reqwest::{Client, StatusCode};

pub async fn get_movie_info_from_server(id: &str) -> Result<Movie, MovieError> {
    let response = Client::new().get("http://127.0.0.1:8081/api/movie_info")
        .query(&[("movie_id", id)])
        .send().await;

    match response {
        Ok(response) => {
            // Error faced by server while querying movie
            if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                return Err(MovieError::new(&response.text().await.unwrap()));
            }
            return Ok(serde_json::from_str(&response.text().await.unwrap()).unwrap())
        },
        // Error with the http request itself
        Err(error) => return Err(MovieError::new(&format!("Reqwest error: {}", &error.to_string())))
    }
}

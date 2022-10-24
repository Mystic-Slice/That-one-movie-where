use crate::{movie::Movie, movie_error::MovieError};

pub async fn get_movie_info(id: &str) -> Result<Movie, MovieError> {

    const API_URL: &str = "https://movie-details1.p.rapidapi.com/imdb_api/movie";
    const API_KEY: &str = "404_API_KEY_MISSING";
    const API_HOST: &str = "movie-details1.p.rapidapi.com";

    let client = reqwest::Client::new();

    let response = client.get(API_URL)
        .query(&[("id", id)])
        .header("X-RapidAPI-Key", API_KEY)
        .header("X-RapidAPI-Host", API_HOST)
        .send().await;
    
    if let Err(error) = response {
        return Err(MovieError::new(&format!("Error in API response: {error}")));
    }

    let response = response.unwrap().text().await;

    if let Err(error) = response {
        return Err(MovieError::new(&format!("Could not convert API response to text: {error}")));
    }
    
    let json_response = json::parse(&response.unwrap());

    if json_response.is_err() {
        return Err(MovieError::new(&format!("Movie with id {id} not found")))
    }

    let json_response = json_response.unwrap();

    let req_keys = vec!["title", "rating", "image", "imdb_type"];
    for key in req_keys {
        if !json_response.has_key(key) {
            return Err(MovieError::new(&format!("Could not get the complete info about the movie with id {id}")))
        }
    }

    if json_response["imdb_type"].to_string() != "movie" {
        return Err(MovieError::new(&format!("{} is not a movie", json_response["title"].to_string())))
    }

    Ok(Movie::new(
        json_response["title"].to_string(),
        json_response["rating"].to_string().parse().unwrap(),
        json_response["image"].to_string(),
    ))
}

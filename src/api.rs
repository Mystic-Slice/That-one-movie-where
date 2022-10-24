use crate::{movie::Movie, movie_error::MovieError};

pub async fn get_movie_info(id: &str) -> Result<Movie, MovieError> {

    const API_URL: &str = "http://www.omdbapi.com/";
    const API_KEY: &str = "404_API_KEY_MISSING";

    let client = reqwest::Client::new();

    let response = client.get(API_URL)
        .query(&[("apikey", API_KEY), ("i", id), ("plot", "full")])
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

    let req_keys = vec!["Title", "imdbRating", "Poster", "Type"];
    for key in req_keys {
        if !json_response.has_key(key) {
            return Err(MovieError::new(&format!("Could not get the complete info about the movie with id {id}")))
        } else if json_response[key] == "N/A" {
            return Err(MovieError::new(&format!("Could not get the complete info about the movie with id {id}")))
        }
    }

    if json_response["Type"].to_string() != "movie" {
        return Err(MovieError::new(&format!("{} is not a movie", json_response["title"].to_string())))
    }

    Ok(Movie::new(
        json_response["Title"].to_string(),
        json_response["imdbRating"].to_string().parse().unwrap(),
        json_response["Poster"].to_string(),
    ))
}

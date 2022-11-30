use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
/// Type to hold all the info about a movie
/// TODO: Add all the other data members
pub struct Movie {
    title: String,
    rating: f32,
    poster_url: String,
    plot: String,
}

impl Movie {
    pub fn new(title: String, rating: f32, poster_url: String, plot: String) -> Self {
        Self {
            title,
            rating,
            poster_url,
            plot
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_rating(&self) -> f32 {
        self.rating
    }

    pub fn get_poster_url(&self) -> String {
        self.poster_url.clone()
    }

    pub fn get_plot(&self) -> String {
        self.plot.clone()
    }
}

impl Default for Movie {
    fn default() -> Self {
        Self {
            title: String::from("No Movie"),
            rating: 0f32,
            poster_url: String::from("https://i.imgflip.com/y57y9.jpg"),
            plot: String::from("No plot found")
        }
    }
}
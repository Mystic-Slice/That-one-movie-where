#[derive(Clone, PartialEq)]
pub struct Movie {
    title: String,
    rating: f32,
    poster_url: String,
}

impl Movie {
    pub fn new(title: String, rating: f32, poster_url: String) -> Self {
        Self {
            title,
            rating,
            poster_url,
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
}

impl Default for Movie {
    fn default() -> Self {
        Self {
            title: String::from("No Movie"),
            rating: 0f32,
            poster_url: String::from("https://i.imgflip.com/y57y9.jpg"),
        }
    }
}
use yew::prelude::*;
use yew_hooks::prelude::*;
use regex::Regex;

use crate::api::get_movie_info_from_server;
use movie_types::movie::Movie;
use movie_types::movie_error::MovieError;

#[derive(Clone, Properties, PartialEq)]
pub struct MovieViewProps {
    pub movie_url: String,
}

#[function_component(MovieView)]
pub fn movie_view(props: &MovieViewProps) -> Html {
    let movie: UseAsyncHandle<Movie, MovieError> = {
        let movie_url = props.movie_url.clone();
        use_async(
            async move {
                // Check if movie id is valid
                let movie_id_loc = Regex::new(r"tt\d+").unwrap().find(&movie_url);
                
                if let Some(movie_id_loc) = movie_id_loc {
                    let movie_id = movie_url[movie_id_loc.start()..movie_id_loc.end()].to_string();
                    get_movie_info_from_server(&movie_id).await
                } else {
                    Err(MovieError::new(&format!("Invalid movie url {movie_url}")))
                }
            }
        )
    };

    {
        let movie = movie.clone();
        // requests API only when movie_url changes
        use_effect_with_deps(
            move |_| {
                movie.run();
                || {}
            }
            ,props.movie_url.clone()
        );
    }

    html! {
        <div>
            {
                if movie.loading {
                    html! { 
                        <div class="loading-container">
                            {"Loading, wait a sec" }
                        </div>
                    }
                } else {
                    html! {
                        <div>
                        {
                            if let Some(error) = &movie.error {
                                html! {
                                    <div class="movie-error">
                                    {error}
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            if movie.error.is_some() {
                                html! {}
                            } else {
                                if let Some(movie) = &movie.data {
                                    html! {
                                        <div class="movie-container">
                                            <div class="movie-details">
                                                <p>{ format!("Movie name: {}", movie.get_title()) }</p>
                                                <p>{ format!("Movie rating: {}", movie.get_rating()) }</p>
                                                <p class="plot">{ format!("{}", movie.get_plot()) }</p>
                                            </div>
                                            <div class="movie-poster">
                                                <img class="poster" src={ movie.get_poster_url() }/>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        }
                        </div>
                    }
                }
            }
        </div>
    }
}
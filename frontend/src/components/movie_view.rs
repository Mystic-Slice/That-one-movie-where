use yew::prelude::*;
use yew_hooks::prelude::*;
use regex::Regex;

use crate::api::get_movie_info_from_server;
use movie_types::movie::Movie;
use movie_types::movie_error::MovieError;

#[derive(Clone, Properties, PartialEq)]
pub struct MovieViewProps {
    pub movie_id: String,
}

#[function_component(MovieView)]
pub fn movie_view(props: &MovieViewProps) -> Html {
    let movie: UseAsyncHandle<Movie, MovieError> = {
        let movie_id = props.movie_id.clone();
        use_async(
            async move {
                // Check if movie id is valid
                if !Regex::new(r"^tt\d+$").unwrap().is_match(&movie_id) {
                    Err(MovieError::new(&format!("Invalid movie id {movie_id}")))
                } else {
                    get_movie_info_from_server(&movie_id).await
                }
            }
        )
    };

    {
        let movie = movie.clone();
        // requests API only when movie_id changes
        use_effect_with_deps(
            move |_| {
                movie.run();
                || {}
            }
            ,props.movie_id.clone()
        );
    }

    html! {
        <div>
            <p>
            {
                if movie.loading {
                    html! { "Loading, wait a sec" }
                } else {
                    html! {
                        <p>
                        {
                            if let Some(error) = &movie.error {
                                html! {
                                    <div>
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
                                        <div>
                                            <p>{ movie.get_title() }</p>
                                            <p>{ movie.get_rating() }</p>
                                            <img src={ movie.get_poster_url() }/>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        }
                        </p>
                    }
                }
            }
            </p>
        </div>
    }
}
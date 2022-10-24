use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::console::log;
use wasm_bindgen::JsCast;

use crate::api::get_movie_info;
use crate::movie::Movie;

pub struct MovieView {
    id: String,
    movie: Movie,
    valid: bool
}

pub enum Msg {
    UpdateId(String),
    FindMovie,
    UpdateViewMovie(Option<Movie>),
}

impl Component for MovieView {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: String::from(""),
            movie: Movie::default(),
            valid: false
        }
    }

    fn update(&mut self, context: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FindMovie => {
                let id = self.id.clone();
                context.link().send_future( async move {
                    let movie = get_movie_info(&id).await;
                    if let Some(movie) = movie {
                        log!(format!("Movie found! Name: {} Rating: {}", movie.get_title(), movie.get_rating()));
                        Msg::UpdateViewMovie(Some(movie))
                    } else {
                        log!(format!("Could not find movie with id {id}"));
                        Msg::UpdateViewMovie(None)
                    }
                });
                false
            },
            Msg::UpdateId(new_id) => {
                self.id = new_id;
                true
            },
            Msg::UpdateViewMovie(movie) => {
                if let Some(movie) = movie {
                    self.valid = true;
                    self.movie = movie;
                } else {
                    self.valid = false;
                    self.movie = Movie::default();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <p>{"Title"}</p>
                <input type="text" oninput={ link.callback(|event: InputEvent| {
                    let target = event.target().unwrap();
                    let input = target.unchecked_into::<HtmlInputElement>();
                    Msg::UpdateId(input.value())
                }) }/>
                <button onclick={link.callback(|_| Msg::FindMovie)}>{"Get movie"}</button>
                if self.valid {
                    <p>{ self.movie.get_title() }</p>
                    <p>{ self.movie.get_rating() }</p>
                    <p>{ &self.id }</p>
                } else {
                    <p>{ "No movie with the given title" }</p>
                }
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
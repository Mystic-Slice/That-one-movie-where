use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use crate::components::movie_view::MovieView;

#[derive(PartialEq)]
enum MovieViewState {
    Closed, // Only input bar and buttons
    Open // Movie info shown
}

impl Default for MovieViewState {
    fn default() -> Self {
        MovieViewState::Closed
    }
}

#[function_component(MainView)]
pub fn main_view() -> Html {
    // need separate movie_url_state to prevent
    // re-renders of MovieView whenever text changes
    let movie_url_state = use_state(|| String::from(""));
    let movie_view_state = use_state(|| MovieViewState::default());
    let text_state = use_state(|| String::from(""));

    // submit button event handler
    // TODO: Error when input not valid
    let handle_submit = {
        let movie_url_state = movie_url_state.clone();
        let movie_view_state = movie_view_state.clone();
        let text_state = text_state.clone();
        Callback::from(move |_| {
            movie_view_state.set(MovieViewState::Open);
            movie_url_state.set((*text_state).clone());
        })
    };

    // text change event handler
    let handle_change_text = {
        let text_state = text_state.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            text_state.set(input.value());
        })
    };

    // close button event handler
    let handle_close = {
        let movie_view_state = movie_view_state.clone();
        Callback::from(move |_| {
            movie_view_state.set(MovieViewState::Closed);
        })
    };

    html! {
        <div>
            <div class="input-container">
                <input id="movie-id" class="text-bar" value={(*text_state).clone()} oninput={handle_change_text} placeholder="IMDB-URL"/>
                <button class="buttons" onclick={handle_submit}>{"Submit"}</button>
                <button class="buttons" onclick={handle_close}>{"Close"}</button>
            </div>
            <div class="movie-view">
            {
                if *movie_view_state == MovieViewState::Open {
                    html! {
                        <MovieView movie_url={(*movie_url_state).clone()}/>
                    }
                } else {
                    html! {}
                }
            }
            </div>
        </div>
    }
}
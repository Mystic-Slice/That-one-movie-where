use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::prelude::*;

use crate::ui::movie_view::MovieView;

#[derive(PartialEq)]
enum MovieViewState {
    Closed,
    Open
}

impl Default for MovieViewState {
    fn default() -> Self {
        MovieViewState::Closed
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // need separate movie_id_state to prevent
    // re-renders of MovieView whenever text changes
    let movie_id_state = use_state(|| String::from(""));
    let movie_view_state = use_state(|| MovieViewState::default());
    let text_state = use_state(|| String::from(""));

    // submit button event handler
    let handle_submit = {
        let movie_id_state = movie_id_state.clone();
        let movie_view_state = movie_view_state.clone();
        let text_state = text_state.clone();
        Callback::from(move |_| {
            log!("Changed text");
            movie_view_state.set(MovieViewState::Open);
            movie_id_state.set((*text_state).clone());
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
            <input type="text" value={(*text_state).clone()} oninput={handle_change_text}/>
            <button onclick={handle_submit}>{"Submit"}</button>
            <button onclick={handle_close}>{"Close"}</button>
            if *movie_view_state == MovieViewState::Open {
                <MovieView movie_id={(*movie_id_state).clone()}/>
            }
        </div>
    }
}
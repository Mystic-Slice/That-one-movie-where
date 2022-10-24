mod api;
mod movie;
mod movie_error;
mod ui;
use ui::app::App;

fn main() {
    yew::start_app::<App>();
}

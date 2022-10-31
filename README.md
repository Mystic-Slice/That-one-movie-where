# That one movie where...

Are you searching for a movie that you forgot the name to? But you remember its IMDB title id. Well, this website is exactly for you!

To host this website locally:
1. Clone the repository
2. Run `dev.bat`

If you are not on Windows, run the commands in `start_frontend.bat` and `start_server.bat` in separate terminals.

Btw, even if that works, the website still won't work as expected. Because I protecc my API_KEY. Try getting your own API_KEY for the [OMDB API](https://www.omdbapi.com/) (its free!) and place in a file `api_key.txt` in the project root.

### Why did I build this website?
No idea. Just wanted to try out Rust and WASM stuff. Figured out how to use async functions too!

### What is it built on?
Uses Axum for backend and Yew for the frontend. Does this website even need a backend? No. #overengineered
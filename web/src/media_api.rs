use std::io;
use std::sync::Arc;

use actix_web::{http, server, App, Path, Responder};
use failure::Error;

use data::{page_movies, get_movie, Movie, get_episode, page_tv_shows, TvShow};
use partial_file::{serve_partial, PartialFile};

#[derive(FromForm)]
pub struct PlayRequest {
    year: Option<u16>,
}

#[get("/play/<title>")]
pub fn play_movie_without_year(state: State<Arc<Vec<Movie>>>, title: String) -> Result<io::Result<PartialFile>, Error> {
    play_movie(state, title, PlayRequest{ year: None })
}

#[get("/play/<title>?<play_request>")]
pub fn play_movie(state: State<Arc<Vec<Movie>>>, title: String, play_request: PlayRequest) -> Result<io::Result<PartialFile>, Error>  {
    let movie = get_movie(state.inner(), &title, play_request.year).ok_or(format_err!("movie not found"))?;
    Ok(serve_partial(Path::new(&movie.file_path)))
}

pub fn movie_routes() -> Vec<Route> {
    routes![all_movies_root, all_movies, play_movie_without_year, play_movie]
}

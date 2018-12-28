use std::sync::Arc;
use actix_web::HttpRequest;
use serde_derive::Serialize;

use crate::data::Movie;
use crate::error::Error;

pub mod view;

/// Error payload for a view (HTML or JSON)
#[derive(Clone, Serialize, Debug)]
struct ErrorPayload {
    message: String,
}

impl ErrorPayload {
    /// Creates a new error payload from a (db.Error.html)
    pub fn from_error(e: &Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Serialize)]
pub struct AllMoviesPayload {
    movies: Arc<Vec<Arc<Movie>>>,
}

/// Represents a payload of verses (HTML or JSON).
#[derive(Clone, Serialize, Debug)]
pub struct MoviePayload<'a> {
    movie: &'a Movie,
}

impl<'a> MoviePayload<'a> {
    /// Creates a new payload for the verses page.
    pub fn new(
        movie: &'a Movie,
        _req: &HttpRequest,
    ) -> Self {
        Self {
            movie,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Meta {
    description: String,
    title: String,
    url: String,
}

macro_rules! title_format {
    () => {
        "Bible.rs | {}"
    };
}

macro_rules! url_format {
    () => {
        "https://bible.rs{}"
    };
}

impl Meta {
    fn for_about() -> Self {
        Self {
            description: "About Bible.rs".to_string(),
            title: format!(title_format!(), "About"),
            url: format!(url_format!(), "/about"),
        }
    }

    fn for_all_movies() -> Self {
        Self {
            description: "All Available Movies.".to_string(),
            title: format!(title_format!(), "Movies"),
            url: format!(url_format!(), ""),
        }
    }

    fn for_movie(movie: &Movie) -> Self {
        Self {
            description: format!("The book of {}", movie.title),
            title: format!(title_format!(), movie.title),
            url: format!(url_format!(), format!("/movie/{}", movie.title)),
        }
    }

    fn for_error() -> Self {
        Self {
            description: "Error page".to_string(),
            title: format!(title_format!(), "Error"),
            url: format!(url_format!(), ""),
        }
    }
}

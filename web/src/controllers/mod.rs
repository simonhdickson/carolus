use std::sync::Arc;
use actix_web::HttpRequest;
use serde_derive::Serialize;

use data::{Movie, TvEpisode, TvSeries, TvShow, {error::Error}};

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
}

#[derive(Clone, Serialize, Debug)]
pub struct AllMoviesPayload {
    movies: Arc<Vec<Arc<Movie>>>,
}

/// Represents a movie payload (HTML or JSON).
#[derive(Clone, Serialize, Debug)]
pub struct MoviePayload<'a> {
    movie: &'a Movie,
}

impl<'a> MoviePayload<'a> {
    /// Creates a new payload for the movie page.
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
pub struct AllTvShowsPayload {
    tv_shows: Arc<Vec<Arc<TvShow>>>,
}

/// Represents a tv show payload (HTML or JSON).
#[derive(Clone, Serialize, Debug)]
pub struct TvShowPayload<'a> {
    tv_show: &'a TvShow,
}

impl<'a> TvShowPayload<'a> {
    /// Creates a new payload for the tv show page.
    pub fn new(
        tv_show: &'a TvShow,
        _req: &HttpRequest,
    ) -> Self {
        Self {
            tv_show,
        }
    }
}

/// Represents a tv series payload (HTML or JSON).
#[derive(Clone, Serialize, Debug)]
pub struct TvSeriesPayload<'a> {
    tv_show: &'a TvShow,
    tv_series: &'a TvSeries,
}

impl<'a> TvSeriesPayload<'a> {
    /// Creates a new payload for the tv series page.
    pub fn new(
        tv_show: &'a TvShow,
        tv_series: &'a TvSeries,
        _req: &HttpRequest,
    ) -> Self {
        Self {
            tv_show,
            tv_series,
        }
    }
}

/// Represents a tv series payload (HTML or JSON).
#[derive(Clone, Serialize, Debug)]
pub struct TvEpisodePayload<'a> {
    tv_show: &'a TvShow,
    tv_series: &'a TvSeries,
    tv_episode: &'a TvEpisode,
}

impl<'a> TvEpisodePayload<'a> {
    /// Creates a new payload for the tv series page.
    pub fn new(
        tv_show: &'a TvShow,
        tv_series: &'a TvSeries,
        tv_episode: &'a TvEpisode,
        _req: &HttpRequest,
    ) -> Self {
        Self {
            tv_show,
            tv_series,
            tv_episode,
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
        "Carolus | {}"
    };
}

macro_rules! url_format {
    () => {
        "https://carolus{}"
    };
}

impl Meta {
    fn for_home() -> Self {
        Self {
            description: "Carolus".to_string(),
            title: format!(title_format!(), "Carolus"),
            url: format!(url_format!(), "/"),
        }
    }

    fn for_about() -> Self {
        Self {
            description: "About Carolus".to_string(),
            title: format!(title_format!(), "About"),
            url: format!(url_format!(), "/about"),
        }
    }

    fn for_all_movies() -> Self {
        Self {
            description: "All Available Movies.".to_string(),
            title: format!(title_format!(), "Movies"),
            url: format!(url_format!(), "/movies"),
        }
    }

    fn for_movie(movie: &Movie) -> Self {
        Self {
            description: movie.title.to_owned(),
            title: format!(title_format!(), movie.title),
            url: format!(url_format!(), format!("/movie/{}", movie.title)),
        }
    }

    fn for_all_tv_shows() -> Self {
        Self {
            description: "All Available TV Shows.".to_string(),
            title: format!(title_format!(), "TV Shows"),
            url: format!(url_format!(), "/tv"),
        }
    }

    fn for_tv_show(tv_show: &TvShow) -> Self {
        Self {
            description: tv_show.title.to_owned(),
            title: format!(title_format!(), tv_show.title),
            url: format!(url_format!(), format!("/tv/{}", tv_show.title)),
        }
    }

    fn for_tv_series(tv_show: &TvShow, tv_series: &TvSeries) -> Self {
        Self {
            description: format!("{}: Series {}", tv_show.title, tv_series.series_number),
            title: format!(title_format!(), format!("{}: Series {}", tv_show.title, tv_series.series_number)),
            url: format!(url_format!(), format!("/tv/{}/{}", tv_show.title, tv_series.series_number)),
        }
    }

    fn for_tv_episode(tv_show: &TvShow, tv_series: &TvSeries, tv_episode: &TvEpisode) -> Self {
        Self {
            description: format!("{}: Series {}", tv_show.title, tv_series.series_number),
            title: format!(title_format!(), format!("{}: Series {}, Episode: {}", tv_show.title, tv_series.series_number, tv_episode.episode_number)),
            url: format!(url_format!(), format!("/tv/{}/{}/{}", tv_show.title, tv_series.series_number, tv_episode.episode_number)),
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

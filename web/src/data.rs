use std::sync::Arc;

use actix_web::actix::*;
use serde_derive::Serialize;

use crate::error::Error;

pub struct DataSet {
    pub movies: Arc<Vec<Arc<Movie>>>,
}

pub struct DataExecutor(pub DataSet);

impl Actor for DataExecutor {
    type Context = SyncContext<Self>;
}

#[derive(PartialEq, Debug, Serialize)]
pub struct Movie {
    pub title: String,
    pub year: Option<u16>,
    pub file_path: String
}

pub struct AllMoviesMessage;

type AllMoviesResult = Result<Arc<Vec<Arc<Movie>>>, Error>;

impl Message for AllMoviesMessage {
    type Result = AllMoviesResult;
}

impl Handler<AllMoviesMessage> for DataExecutor {
    type Result = AllMoviesResult;

    fn handle(&mut self, _: AllMoviesMessage, _: &mut Self::Context) -> Self::Result {
        Ok(self.0.movies.clone())
    }
}

pub struct MovieMessage {
    pub title: String,
    pub year: Option<u16>,
}

type MovieResult = Result<Arc<Movie>, Error>;

impl Message for MovieMessage {
    type Result = MovieResult;
}

impl Handler<MovieMessage> for DataExecutor {
    type Result = MovieResult;

    fn handle(&mut self, msg: MovieMessage, _: &mut Self::Context) -> Self::Result {
        match self.0.movies.iter().find(|m|m.title.eq_ignore_ascii_case(&*msg.title) && m.year == msg.year) {
            None => self.0.movies.iter().find(|m|m.title.eq_ignore_ascii_case(&*msg.title)),
            movie => movie,
        }
        .cloned()
        .ok_or_else(||Error::MovieNotFound{ title: msg.title })
    }
}

pub fn get_movie<'a>(movies: &'a Vec<Movie>, title: &str, year: Option<u16>) -> Result<&'a Movie, Error> {
    match movies.iter().find(|m|m.title.eq_ignore_ascii_case(title) && m.year == year) {
        None => movies.iter().find(|m|m.title.eq_ignore_ascii_case(title)),
        movie => movie,
    }.ok_or_else(||Error::MovieNotFound{ title: title.to_owned() })
}

#[derive(PartialEq, Debug)]
pub struct TvShow {
    pub title: String,
    pub year: Option<u16>,
    pub series: Vec<TvSeries>
}

#[derive(PartialEq, Debug)]
pub struct TvSeries {
    pub series_number: u16,
    pub episodes: Vec<TvEpisode>
}

#[derive(PartialEq, Debug)]
pub struct TvEpisode {
    pub episode_number: u16,
    pub file_path: String,
}

pub fn page_tv_shows<'a>(tv_shows: &'a Vec<TvShow>, page: i64, count: i64) -> Option<&'a [TvShow]> {
    tv_shows.chunks(count as usize).skip(page as usize).next()
}

pub fn get_episode<'a> (tv_shows: &'a Vec<TvShow>, title: &str, year: Option<u16>, series: u16, episode: u16) -> Option<(&'a TvShow, &'a TvSeries, &'a TvEpisode)> {
    let tv_show =
        match tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(title) && s.year == year) {
            None => tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(title)),
            tv_show => tv_show,
        }?;
    let series = tv_show.series.iter().find(|s|s.series_number == series)?;
    let episode = series.episodes.iter().find(|s|s.episode_number == episode)?;
    Some((tv_show, series, episode))
}

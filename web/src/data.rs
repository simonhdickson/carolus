use std::sync::Arc;

use actix_web::actix::*;
use serde_derive::Serialize;

use crate::error::Error;

pub struct DataSet {
    pub movies: Arc<Vec<Arc<Movie>>>,
    pub tv_shows: Arc<Vec<Arc<TvShow>>>,
}

pub struct DataExecutor(pub DataSet);

impl Actor for DataExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TvShow {
    pub title: String,
    pub year: Option<u16>,
    pub series: Vec<TvSeries>
}

pub struct AllTvShowsMessage;

type AllTvShowsResult = Result<Arc<Vec<Arc<TvShow>>>, Error>;

impl Message for AllTvShowsMessage {
    type Result = AllTvShowsResult;
}

impl Handler<AllTvShowsMessage> for DataExecutor {
    type Result = AllTvShowsResult;

    fn handle(&mut self, _: AllTvShowsMessage, _: &mut Self::Context) -> Self::Result {
        Ok(self.0.tv_shows.clone())
    }
}

pub struct TvShowMessage {
    pub title: String,
    pub year: Option<u16>,
}

type TvShowResult = Result<Arc<TvShow>, Error>;

impl Message for TvShowMessage {
    type Result = TvShowResult;
}

impl Handler<TvShowMessage> for DataExecutor {
    type Result = TvShowResult;

    fn handle(&mut self, msg: TvShowMessage, _: &mut Self::Context) -> Self::Result {
        match self.0.tv_shows.iter().find(|m|m.title.eq_ignore_ascii_case(&*msg.title) && m.year == msg.year) {
            None => self.0.tv_shows.iter().find(|m|m.title.eq_ignore_ascii_case(&*msg.title)),
            tv_show => tv_show,
        }
        .cloned()
        .ok_or_else(||Error::TvShowNotFound{ title: msg.title })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TvSeries {
    pub series_number: u16,
    pub episodes: Vec<TvEpisode>
}

pub struct TvSeriesMessage {
    pub title: String,
    pub year: Option<u16>,
    pub series: u16,
}

type TvSeriesResult = Result<(Arc<TvShow>, TvSeries), Error>;

impl Message for TvSeriesMessage {
    type Result = TvSeriesResult;
}

impl Handler<TvSeriesMessage> for DataExecutor {
    type Result = TvSeriesResult;

    fn handle(&mut self, msg: TvSeriesMessage, _: &mut Self::Context) -> Self::Result {
        match self.0.tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(&*msg.title) && s.year == msg.year) {
            None => self.0.tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(&*msg.title)),
            tv_show => tv_show,
        }
        .and_then(|tv_show|{
            let series = tv_show.series.iter().find(|s|s.series_number == msg.series)?;
            Some((tv_show.clone(), series.clone()))
        })
        .ok_or_else(||Error::TvShowNotFound{ title: msg.title })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TvEpisode {
    pub episode_number: u16,
    pub file_path: String,
}

pub struct TvEpisodeMessage {
    pub title: String,
    pub year: Option<u16>,
    pub series: u16,
    pub episode: u16,
}

type TvEpisodeResult = Result<(Arc<TvShow>, TvSeries, TvEpisode), Error>;

impl Message for TvEpisodeMessage {
    type Result = TvEpisodeResult;
}

impl Handler<TvEpisodeMessage> for DataExecutor {
    type Result = TvEpisodeResult;

    fn handle(&mut self, msg: TvEpisodeMessage, _: &mut Self::Context) -> Self::Result {
        match self.0.tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(&*msg.title) && s.year == msg.year) {
            None => self.0.tv_shows.iter().find(|s|s.title.eq_ignore_ascii_case(&*msg.title)),
            tv_show => tv_show,
        }
        .and_then(|tv_show|{
            let series = tv_show.series.iter().find(|s|s.series_number == msg.series)?;
            Some((tv_show, series))
        })
        .and_then(|(tv_show, tv_series)|{
            let tv_episode = tv_series.episodes.iter().find(|s|s.episode_number == msg.episode)?;
            Some((tv_show.clone(), tv_series.clone(), tv_episode.clone()))
        })
        .ok_or_else(||Error::TvShowNotFound{ title: msg.title })
    }
}

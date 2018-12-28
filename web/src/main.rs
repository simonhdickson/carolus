
use std::sync::Arc;

use actix_web::{
    actix::*,
    fs,
    middleware, server, App,
};
use failure::Error;
use handlebars::Handlebars;
use log::{info, Level};

use crate::controllers::{view};
use crate::data::{DataExecutor, DataSet, Movie, TvShow, TvSeries, TvEpisode};

mod cli;
mod controllers;
mod data;
mod error;
mod file_index;

pub struct ServerState {
    pub data: Addr<DataExecutor>,
    pub template: Handlebars,
}

/// Registers the [Handlebars](handlebars.handlebars.html) templates for the application.
fn register_templates() -> Result<Handlebars, Error> {
    let mut tpl = Handlebars::new();
    tpl.set_strict_mode(true);
    tpl.register_templates_directory(".hbs", "./web/templates/")?;

    Ok(tpl)
}

#[derive(Default)]
struct StaticFileConfig;

impl fs::StaticFileConfig for StaticFileConfig {
    fn is_use_etag() -> bool {
        true
    }
}

fn get_demo_data_set() -> (Vec<Movie>, Vec<TvShow>) {
    (vec![
        Movie {
            title: "Die Hard".to_owned(),
            year: None,
            file_path: "./web/dist/video/demo.mp4".to_owned(),
        }
    ], vec![
        TvShow {
            title: "Jonathan Creek".to_owned(),
            year: None,
            series: vec![TvSeries {
                series_number: 1,
                episodes: vec![
                    TvEpisode {
                        episode_number: 1,
                        file_path: "./web/dist/video/demo.mp4".to_owned(),
                    }
                ],
            }],
        }
    ])
}

fn main() -> Result<(), Error> {
    let matches = cli::build_cli().get_matches();

    init_logging(matches.occurrences_of("v"))?;

    let (movies, tv_shows) = if matches.is_present("demo") {
        get_demo_data_set()
    } else {
         file_index::index::index(matches.value_of("movie_path"), matches.value_of("tv_path"))?
    };

    info!("finished indexing files");
    
    let movies = Arc::new(movies.into_iter().map(|m|Arc::new(m)).collect::<Vec<_>>());
    let tv_shows = Arc::new(tv_shows.into_iter().map(|m|Arc::new(m)).collect::<Vec<_>>());
    
    let sys = System::new("carolus");
    let addr = SyncArbiter::start(num_cpus::get(), move || DataExecutor(DataSet{movies: movies.clone(), tv_shows: tv_shows.clone()}));

    server::new(move || {
        let template = register_templates().unwrap();

        App::with_state(ServerState {
            data: addr.clone(),
            template,
        })
        .handler(
            "/static",
            fs::StaticFiles::with_config("./web/dist", StaticFileConfig).unwrap(),
        )
        .resource("/", |r| r.get().with(view::home))
        .resource("/about", |r| r.get().with(view::about))
        .resource("/movies", |r| {
            r.name("all_movies");
            r.get().with(view::all_movies)
        })
        .resource("/movie/{movie}", |r| {
            r.name("movie");
            r.get().f(view::movie)
        })
        .resource("play/movie/{movie}", |r| {
            r.get().f(view::play_movie)
        })
        .resource("/tv", |r| {
            r.name("all_tv_shows");
            r.get().with(view::all_tv_shows)
        })
        .resource("/tv/{tv_show}", |r| {
            r.name("tv_show");
            r.get().f(view::tv_show)
        })
        .resource("/tv/{tv_show}/{series}", |r| {
            r.name("tv_series");
            r.get().f(view::tv_series)
        })
        .resource("/tv/{tv_show}/{series}/{episode}", |r| {
            r.name("tv_episode");
            r.get().f(view::tv_episode)
        })
        .resource("/play/tv/{tv_show}/{series}/{episode}", |r| {
            r.get().f(view::play_tv_episode)
        })
        .middleware(middleware::Logger::default())
    })
    .bind(format!("0.0.0.0:{}", matches.value_of("port").unwrap()))
    .unwrap()
    .start();

    let _ = sys.run();

    Ok(())
}

fn init_logging(level: u64) -> Result<(), Error> {
    let log_level =
        match level {
            0 => Level::Warn,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        };

    simple_logger::init_with_level(log_level)?;
    Ok(())
}

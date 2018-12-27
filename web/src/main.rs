use crate::data::{Movie, TvShow};
use std::sync::Arc;
use std::time::Instant;

use actix_web::{
    actix::*,
    fs,
    http::{ContentEncoding, Method, NormalizePath},
    middleware, server, App,
};
use clap::{Arg, App as ClapApp};
use failure::Error;
use handlebars::Handlebars;
use log::Level;

use crate::controllers::{view};

mod controllers;
mod data;
mod error;
mod file_index;

pub struct ServerState {
    pub movies: Arc<Vec<Movie>>,
    pub tv_shows: Arc<Vec<TvShow>>,
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

fn main() -> Result<(), Error> {
    let matches =
        ClapApp::new("carolus")
            .version("0.1.0")
            .about("Open Source Multimedia Server")
            .author("Simon Dickson")
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .arg(Arg::with_name("movie_path")
                .short("mp")
                .env("CAROLUS_MOVIES_PATH")
                .help("Sets the movie directory"))
            .arg(Arg::with_name("tv_path")
                .short("tp")
                .env("CAROLUS_TV_PATH")
                .help("Sets the tv directory"))
            .get_matches();

    init_logging(matches.occurrences_of("v"))?;

    let (movies, tv_shows) = file_index::index::index(matches.value_of("movie_path"), matches.value_of("tv_path"))?;
    let movies = Arc::new(movies);
    let tv_shows = Arc::new(tv_shows);
    
    let sys = System::new("carolus");

    server::new(move || {
        let template = register_templates().unwrap();

        App::with_state(ServerState {
            movies: movies.clone(),
            tv_shows: tv_shows.clone(),
            template,
        })
        .default_encoding(ContentEncoding::Gzip)
        //.handler(
        //    "/static",
        //    fs::StaticFiles::with_config("./web/dist", StaticFileConfig).unwrap(),
        //)
        .resource("about", |r| r.get().with(view::about))
        .middleware(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")
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

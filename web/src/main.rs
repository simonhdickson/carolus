
use std::sync::Arc;

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
use crate::data::{DataExecutor, DataSet, Movie, TvShow};

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
            file_path: "./fail".to_owned(),
        }
    ], vec![])
}

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
            .arg(Arg::with_name("demo")
                .long("demo")
                .help("Sets the tv directory"))
            .get_matches();

    init_logging(matches.occurrences_of("v"))?;

    let (movies, _tv_shows) = if matches.is_present("demo") {
        get_demo_data_set()
    } else {
         file_index::index::index(matches.value_of("movie_path"), matches.value_of("tv_path"))?
    };
    
    let movies = Arc::new(movies.into_iter().map(|m|Arc::new(m)).collect::<Vec<_>>());
    //let tv_shows = Arc::new(tv_shows);
    
    let sys = System::new("carolus");
    let addr = SyncArbiter::start(num_cpus::get(), move || DataExecutor(DataSet{movies: movies.clone()}));

    server::new(move || {
        let template = register_templates().unwrap();

        App::with_state(ServerState {
            data: addr.clone(),
            template,
        })
        .default_encoding(ContentEncoding::Gzip)
        .handler(
            "/static",
            fs::StaticFiles::with_config("./web/dist", StaticFileConfig).unwrap(),
        )
        .resource("about", |r| r.get().with(view::about))
        .resource("/movies", |r| {
            r.name("all_movies");
            r.get().with(view::all_movies)
        })
        .resource("/movie/{movie}", |r| {
            r.name("movie");
            r.get().f(view::movie)
        })
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

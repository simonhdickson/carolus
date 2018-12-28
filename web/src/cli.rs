use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("carolus")
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
}

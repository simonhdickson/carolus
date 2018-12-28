use failure::Fail;

/// Error type that for the Carolus application.
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(
        display = "There was an error with the Actix async arbiter. Cause: {}",
        cause
    )]
    Actix { cause: String },

    #[fail(display = "'{}' was not found.", title)]
    MovieNotFound { title: String },

    #[fail(display = "'{}' was not found.", title)]
    TvShowNotFound { title: String },

    #[fail(display = "There was an error rendering the HTML page.")]
    Template,
}

use actix_web::*;
use actix_web::actix::*;
use failure::{self, Fail};
use futures::future::{err, Future};
use handlebars::Handlebars;
use lazy_static::lazy_static;
use log::error;
use serde::Serialize;
use serde_derive::Serialize;

use crate::data::{AllMoviesMessage, MovieMessage};
use crate::controllers::*;
use crate::error::Error;
use crate::ServerState;

lazy_static! {
    static ref ERR_TPL: Handlebars = {
        let mut tpl = Handlebars::new();
        tpl.register_template_file("base", "./web/templates/base.hbs")
            .unwrap();
        tpl.register_template_file("error", "./web/templates/error.hbs")
            .unwrap();
        tpl
    };
}

#[derive(Serialize)]
struct TemplatePayload<T: Serialize> {
    data: T,
    meta: Meta,
}

impl<T: Serialize> TemplatePayload<T> {
    /// Create a new HTML template payload.
    fn new(data: T, meta: Meta) -> Self {
        Self { data, meta }
    }

    /// Convert the template payload to HTML
    fn to_html(&self, tpl_name: &str, renderer: &Handlebars) -> Result<String, Error> {
        renderer.render(tpl_name, &self).map_err(|e| {
            error!("{}", e);
            Error::Template
        })
    }
}

#[derive(Fail, Debug)]
#[fail(display = "HTML Error")]
pub struct HtmlError(Error);

impl From<Error> for HtmlError {
    /// Transforms an HtmlError into an actix_web HTTP Response.
    fn from(f: Error) -> Self {
        HtmlError(f)
    }
}

impl error::ResponseError for HtmlError {
    fn error_response(&self) -> HttpResponse {
        let body = &TemplatePayload::new(ErrorPayload::from_error(&self.0), Meta::for_error())
            .to_html("error", &ERR_TPL)
            .unwrap();

        match self.0 {
            Error::Actix { .. } | Error::Db | Error::Template => {
                HttpResponse::InternalServerError()
            }
            Error::MovieNotFound { .. } => HttpResponse::NotFound(),
            Error::InvalidReference { .. } => HttpResponse::BadRequest(),
        }
        .content_type("text/html")
        .body(body)
    }
}

impl From<MailboxError> for HtmlError {
    fn from(e: MailboxError) -> Self {
        HtmlError(Error::Actix {
            cause: e.to_string(),
        })
    }
}

type AsyncResponse = Box<dyn Future<Item = HttpResponse, Error = HtmlError>>;

#[derive(Serialize)]
struct EmptyPayload;

pub fn home((state,): (State<ServerState>,)) -> Result<HttpResponse, HtmlError> {
    let body =
        TemplatePayload::new(EmptyPayload, Meta::for_home()).to_html("home", &state.template)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn about((state,): (State<ServerState>,)) -> Result<HttpResponse, HtmlError> {
    let body =
        TemplatePayload::new(EmptyPayload, Meta::for_about()).to_html("about", &state.template)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn all_movies((state,): (State<ServerState>,)) -> AsyncResponse {
    state
        .data
        .send(AllMoviesMessage)
        .from_err()
        .and_then(move |res| match res {
            Ok(movies) => {
                let body = TemplatePayload::new(AllMoviesPayload { movies }, Meta::for_all_movies())
                    .to_html("all-movies", &state.template)?;

                Ok(HttpResponse::Ok().content_type("text/html").body(body))
            }
            Err(e) => Err(HtmlError(e)),
        })
        .responder()
}

pub fn movie(req: &HttpRequest<ServerState>) -> AsyncResponse {
    let info = Path::<(String,)>::extract(req).unwrap();
    let data = &req.state().data;

    let req = req.to_owned();
    data.send(MovieMessage {
        title: info.0.to_owned(),
        year: None,
    })
    .from_err()
    .and_then(move |res| match res {
        Ok(result) => {
            let payload = MoviePayload::new(&result, &req.drop_state());
            let body = TemplatePayload::new(
                &payload,
                Meta::for_movie(&payload.movie),
            )
            .to_html("movie", &req.state().template)?;

            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        Err(e) => Err(HtmlError(e)),
    })
    .responder()
}

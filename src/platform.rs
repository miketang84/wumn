use cfg_if::cfg_if;
use crate::{
    error::ParseError,
    Database,
};
//use log::*;
use std::{
    convert::TryFrom,
    ops::Deref,
};
use url::Url;

cfg_if! {if #[cfg(feature = "with-postgres")]{
    use crate::pg::PostgresDB;
}}

pub enum DBPlatform {
    #[cfg(feature = "with-postgres")]
    //Postgres(PostgresDB),
    Postgres(Box<PostgresDB>),
}

impl Deref for DBPlatform {
    type Target = dyn Database;

    fn deref(&self) -> &Self::Target {
        match *self {
            #[cfg(feature = "with-postgres")]
            DBPlatform::Postgres(ref pg) => pg.deref(),
        }
    }
}

pub(crate) enum Platform {
    #[cfg(feature = "with-postgres")]
    Postgres,
    Unsupported(String),
}

impl<'a> TryFrom<&'a str> for Platform {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let url = Url::parse(s);
        match url {
            Ok(url) => {
                let scheme = url.scheme();
                match scheme {
                    #[cfg(feature = "with-postgres")]
                    "postgres" => Ok(Platform::Postgres),
                    _ => Ok(Platform::Unsupported(scheme.to_string())),
                }
            }
            Err(e) => Err(ParseError::DbUrlParseError(e)),
        }
    }
}

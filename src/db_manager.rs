use entity::EntityManager;
use error::DbError;
use error::{ConnectError};
use platform::DBPlatform;
use platform::Platform;
use dao_manager::DaoManager;
use std::convert::TryFrom;

cfg_if! {if #[cfg(feature = "with-postgres")]{
    use pg::{self, PostgresDB};
}}


pub struct DbManager;

impl DbManager {
    pub fn new() -> Self {
        DbManager
    }

    /// ensure that a connection pool for this db_url exist
    fn db(&mut self, db_url: &str) -> Result<DBPlatform, DbError> {
        info!("ensure db_url: {}", db_url);
        let platform: Result<Platform, _> = TryFrom::try_from(db_url);
        match platform {
            Ok(platform) => match platform {
                #[cfg(feature = "with-postgres")]
                Platform::Postgres => {
                    let conn = pg::init_connection(db_url);
                    Ok(DBPlatform::Postgres(PostgresDB(conn)))
                },
                Platform::Unsupported(scheme) => {
                    info!("unsupported");
                    Err(DbError::ConnectError(ConnectError::UnsupportedDb(scheme)))
                }
            },
            Err(e) => Err(DbError::ConnectError(ConnectError::ParseError(e))),
        }
    }

    pub fn em(&mut self, db_url: &str) -> Result<EntityManager, DbError> {
        let db = self.db(db_url)?;
        Ok(EntityManager(db))
    }

    pub fn dm(&mut self, db_url: &str) -> Result<DaoManager, DbError> {
        let db = self.db(db_url)?;
        Ok(DaoManager(db))
    }
}


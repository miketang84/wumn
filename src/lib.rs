//! 
//! Wumn is a thin abstract ORM over SQL and Rust types.
//!
//! ### Selecting records
//!
//! ```rust
//! #[macro_use]
//! use wumn::{ToColumnNames, ToTableName, FromDao, ToDao, DbError, DbManager};
//! 
//! #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
//! struct Actor {
//!     actor_id: i32,
//!     first_name: String,
//! }
//! 
//! fn main(){
//!     let db_url = "postgres://postgres:p0stgr3s@localhost/wumn";
//!     let mut dbm = DbManager::new();
//!     let em = dbm.em(db_url).unwrap();
//!     let sql = "SELECT * FROM actor LIMIT 10";
//!     let actors: Result<Vec<Actor>, DbError> = em.execute_sql_with_return(sql, &[]);
//!     info!("Actor: {:#?}", actors);
//!     let actors = actors.unwrap();
//!     assert_eq!(actors.len(), 10);
//!     for actor in actors {
//!         info!("actor: {:?}", actor);
//!     }
//! }
//! ```
//! ### Inserting and displaying the inserted records
//!
//! ```rust
//! use wumn::{ToColumnNames, ToTableName, FromDao, ToDao, DbError, DbManager};
//! use chrono::{offset::Utc, DateTime, NaiveDate};
//!
//!   fn main() {
//!       mod for_insert {
//!           use super::*;
//!           #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
//!           pub struct Actor {
//!               pub first_name: String,
//!               pub last_name: String,
//!           }
//!       }
//!
//!       mod for_retrieve {
//!           use super::*;
//!           #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
//!           pub struct Actor {
//!               pub actor_id: i32,
//!               pub first_name: String,
//!               pub last_name: String,
//!               pub last_update: DateTime<Utc>,
//!           }
//!       }
//!
//!       let db_url = "postgres://postgres:p0stgr3s@localhost/wumn";
//!       let mut dbm = DbManager::new();
//!       let em = dbm.em(db_url).unwrap();
//!       let tom_cruise = for_insert::Actor {
//!           first_name: "TOM".into(),
//!           last_name: "CRUISE".to_string(),
//!       };
//!       let tom_hanks = for_insert::Actor {
//!           first_name: "TOM".into(),
//!           last_name: "HANKS".to_string(),
//!       };
//!
//!       let actors: Result<Vec<for_retrieve::Actor>, DbError> =
//!           em.insert(&[&tom_cruise, &tom_hanks]);
//!       info!("Actor: {:#?}", actors);
//!       assert!(actors.is_ok());
//!       let actors = actors.unwrap();
//!       let today = Utc::now().date();
//!       assert_eq!(tom_cruise.first_name, actors[0].first_name);
//!       assert_eq!(tom_cruise.last_name, actors[0].last_name);
//!       assert_eq!(today, actors[0].last_update.date());
//!       assert_eq!(tom_hanks.first_name, actors[1].first_name);
//!       assert_eq!(tom_hanks.last_name, actors[1].last_name);
//!       assert_eq!(today, actors[1].last_update.date());
//!   }
//! ```
//!

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-postgres")]{
extern crate postgres;
#[macro_use]
extern crate postgres_shared;
mod pg;
}}

pub mod common;
pub mod column;
mod dao_manager;
mod db_manager;
mod database;
mod entity;
mod platform;
mod users;
pub mod error;
pub mod table;
pub mod types;
pub mod util;

pub use column::Column;
pub use dao_manager::DaoManager;
pub use db_manager::DbManager;
pub use database::{
    Database,
    DatabaseName,
};
pub use entity::EntityManager;
pub use error::{
    DataError,
    DbError,
    PlatformError,
};
pub use table::Table;

// we export the traits that has a derived proc macro
// this are used in the apps
pub use codegen::{
    FromDao,
    ToColumnNames,
    ToDao,
    ToTableName,
};

pub use wumn_dao::{
    ColumnName,
    Dao,
    Rows,
    TableName,
    ToValue,
    Value,
    Array,
};

/// Wrap the wumn_dao exports to avoid name conflict with the wumn_codegen
pub mod dao {
    pub use wumn_dao::{
        FromDao,
        ToColumnNames,
        ToDao,
        ToTableName,
    };
}

/// Wrap the wumn_codegen exports to avoid name conflict with the wumn_dao
pub mod codegen {
    pub use wumn_codegen::{
        FromDao,
        ToColumnNames,
        ToDao,
        ToTableName,
    };
}

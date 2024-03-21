#![allow(dead_code , unused_variables)]

pub use auth_utils::models::Credentials;

pub fn authenticate(creds:Credentials){
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}

mod database;
mod auth_utils;
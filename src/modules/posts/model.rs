use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::modules::posts::schema::posts;

#[derive(Debug, PartialEq)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
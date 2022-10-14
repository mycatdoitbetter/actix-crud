use diesel::{prelude::*, result::Error};

use crate::{
    models::{NewPost, Post},
    postgres::establish_connection,
    schema::posts,
    schema::posts::dsl::*,
};

pub fn read_posts() -> Result<Vec<Post>, Error> {
    let conn = &mut establish_connection();

    posts.filter(published.eq(false)).load::<Post>(conn)
}

pub fn create_post(post_title: &str, post_body: &str) -> Result<Post, Error> {
    let conn = &mut establish_connection();

    let new_post = NewPost {
        title: post_title,
        body: post_body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn update_post(id_to_update: i32, post_title: &str, post_body: &str) -> Result<usize, Error> {
    let conn = &mut establish_connection();

    let post = posts.filter(id.eq(id_to_update)).first::<Post>(conn);

    match post {
        Ok(post) => 
            diesel::update(posts::table)
                .filter(id.eq(post.id))
                .set((title.eq(post_title), body.eq(post_body)))
                .execute(conn)
        ,
        Err(_) => Err(Error::NotFound),
    }
}

pub fn delete_post(id_to_delete: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();

    let post = posts.filter(id.eq(id_to_delete)).first::<Post>(conn);

    match post {
        Ok(post) => diesel::delete(posts.find(post.id)).execute(conn),
        Err(_) => Err(Error::NotFound),
    }
}

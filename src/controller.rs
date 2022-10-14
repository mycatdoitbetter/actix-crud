
use actix_web::{Responder, get, web, HttpResponse, post, patch, delete};

use crate::{
  models::CreatePost, 
  services::{ create_post, read_posts, update_post, delete_post}
};


#[post("posts/")]
async fn create(post: web::Json<CreatePost>) -> impl Responder {
    let post = create_post(&post.title, &post.body);
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[get("posts/")]
async fn read() -> impl Responder {
    let posts = read_posts();

    match posts {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
        
}


#[patch("posts/{id}")]
async fn update(id: web::Path<i32>, post: web::Json<CreatePost>) -> impl Responder {
  let update = update_post(*id, &post.title, &post.body);

  match update {
    Ok(_) => HttpResponse::Ok().body(format!("Post with id {id} updated.")),
    Err(_) => HttpResponse::NotFound().body(format!("Post with {id} not found."))
  }
}       

#[delete("posts/{id}")]
async fn delete(id: web::Path<i32>) -> impl Responder {
  let delete = delete_post(*id);

  match delete {
    Ok(_) => HttpResponse::Ok().body(format!("Post with id {id} deleted.")),
    Err(_) => HttpResponse::NotFound().body(format!("Post with {id} not found."))
  }
}       


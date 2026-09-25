pub mod db_service;
pub mod file_service;
pub mod constants;
use std::path::Path;
use std::process::{Command,Output};
use axum::{Router, routing::get,routing::put,debug_handler,Json,response::Html};
use tower_http::services::ServeDir;

use tokio; 

#[tokio::main]
async fn main() {
    db_service::init().expect("");
    file_service::update_movies(Path::new(constants::movie_dir)).await.expect("file servic failed");
    println!("finished setup now running server");
    let static_files = ServeDir::new(constants::movie_dir);
    let app = Router::new()
        .route("/", get(index))
        .route("/get_all_mov", get(get_all))
        .route("/dl_movie/{url}",put(dl_movie))
        .route("/convert",get(convert))
        .nest_service("/static",static_files); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
#[debug_handler]
async fn get_all() -> Json<Vec<db_service::Movie>>{
    Json(db_service::get_all_mov().expect(""))

}

async fn index() -> Html<String> {
    tokio::spawn(async move{
        file_service::update_movies(Path::new(constants::movie_dir)).await.expect("failed to fileservice");
    });
    let html = tokio::fs::read_to_string("index.html").await.expect("");
    Html(html)

}

#[debug_handler]
async fn dl_movie(url: axum::extract::Path<String>) {

    //Command::new("echo").args([&url.0]).spawn().expect("Error here ");
    
    Command::new("transmission-remote").args(["-a",&url.0]).spawn().expect("Error here ");
}

#[debug_handler]
async fn convert(){
    
    Command::new(constants::path_to_script).args([constants::movie_dir]).spawn().expect("Error here ");
    
}



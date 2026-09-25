use std::fs;
use tokio::join;
use futures;
use std::path::Path;
pub use crate::db_service;
pub use crate::constants::{movie_dir,db_path};
use anyhow::Error;



pub async fn update_movies(p_movie_path: &Path) ->Result<(),anyhow::Error>{
    let mut futs = Vec::new();
    for entry in fs::read_dir(p_movie_path)?{
        let path = entry?.path();
            if path.is_dir(){
                // rust Book Chapter Workaround to love and know 
                Box::pin(update_movies(&path)).await?;
            }
            else{
                if let Some(ext) = path.extension(){
                    if ext == "mp4"{
                        if let Some(path) = path.to_str(){
                            let file_name = match path.strip_prefix(movie_dir){ 
                                        Some(t) => String::from(t),
                                        None => anyhow::bail!("Error while trying to get file_name")
                            };
                            if !db_service::exists_mov(&file_name).expect(""){
                                let mut name = String::from("");
                                for part in path.split("/"){
                                    name = String::from(part);

                                }
                                let mov = db_service::Movie{
                                    name,
                                    file_name,
                                    ..Default::default()

                                };
                                futs.push(db_service::add_mov(mov));
                            };
                            
                        }
                    }

                }
            }
    }
    futures::future::join_all(futs).await;
    Ok(())

}

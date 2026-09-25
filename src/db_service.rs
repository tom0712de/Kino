use rusqlite::{Connection, Result};
use reqwest;
use anyhow::{Error};
use serde::{Serialize,Deserialize};
pub use crate::constants::{movie_dir,db_path,api_key};
use std::path::Path;
#[derive(Default,Debug,Serialize,Deserialize)]
pub struct Movie{
    pub id : i64, 
    pub name: String,
    pub file_name: String,
    pub year: String,
    pub genres: Vec<String>,
    pub imdb_id: String, 
    pub plot: String,
}
impl Movie{

    pub async fn fetch_info(& mut self) -> Result<(),Error>{
        let (name, _) = &self.name.rsplit_once(".").expect("");

        let url = format!("http://omdbapi.com/?s={name}&apikey={api_key}");
        let temp = reqwest::get(url).await?.json::<response>().await?;
       

        //could implement cleaner 
        self.name = temp.Search[0].Title.clone();
        self.year = temp.Search[0].Year.clone();
        self.imdb_id= temp.Search[0].imdbID.clone();

        dbg!(&temp.Search[0].imdbID);
        let id = &temp.Search[0].imdbID;
        let url = format!("http://omdbapi.com/?i={id}&apikey={api_key}");
        let temp = reqwest::get(url).await?.json::<mov_by_id>().await?;

        self.plot = temp.Plot.clone();
        self.genres = temp.Genre.split(", ").map(|s| s.to_string()).collect();

        dbg!(&temp);

        Ok(())
        


    }


}


#[derive(Default,Debug,Serialize,Deserialize)]
pub struct mov_from_api_search{
    pub Title: String,
    pub Year: String,
    pub imdbID: String,


}

#[derive(Default,Debug,Serialize,Deserialize)]
pub struct mov_by_id{
    Title: String,
    Year: String,
    Genre: String,
    Plot: String,


}
#[derive(Default,Debug,Serialize,Deserialize)]
pub struct response{
   pub Search : Vec<mov_from_api_search>, 

}




// Initial DB Setup
pub fn init() ->Result<(),Error>{
    let conn = Connection::open(db_path)?;

    conn.execute(
        "Create TABLE IF NOT EXISTS movies(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            file_name TEXT NOT NULL,
            year TEXT,
            imdb_id TEXT,
            plot TEXT


        );",())?;
    conn.execute(
        "Create Table IF NOT Exists genres(
            mov_name INTEGER NOT NULL,
            genre_name TEXT NOT NULL

            );",())?;
    Ok(())
}
pub fn exists_mov(path: &str) -> Result<bool,Error>{
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("
        SELECT * FROM movies
        WHERE file_name = ?1;
        ")?;
    let mut  rows = stmt.query([path])?;
    match rows.next()?{ 
        Some(_row) =>{
            Ok(true)
            },
        None => Ok(false)

    }
}
pub async fn add_mov(mut mov: Movie) -> Result<(),Error>{
    dbg!(&mov); 
    match mov.fetch_info().await{
        Ok(_t) => (),
        Err(e) => println!("{}",e),
    }

    let conn = Connection::open(db_path)?;
    conn.execute("
    INSERT INTO movies
    (name, file_name,year,imdb_id,plot) 
    VALUES (?1,?2,?3,?4,?5);"
    ,(&mov.name,&mov.file_name,&mov.year,&mov.imdb_id,&mov.plot)
    )?;
    for genre in mov.genres{
        conn.execute("INSERT INTO genres
            (mov_name, genre_name)
            VALUES(?1,?2);"
            ,(&mov.name,genre)
            )?;

    }
    Ok(())
}
pub fn get_all_mov() -> Result<Vec<Movie>,Error>{
    let mut result: Vec<Movie> = vec![];
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("
        SELECT * From movies; 
        ")?;
    let mut rows = stmt.query([])?;

    while let Some(row) =  rows.next()?{
        let mut mov = Movie{
            id : row.get(0)?,
            name: row.get(1)?,
            file_name : row.get(2)?,
            year: row.get(3)?,
            imdb_id: row.get(4)?,
            plot: row.get(5)?,
            ..Default::default()

        };
        result.push(mov);
    }
    for mov in result.iter_mut(){
        let mut stmt = conn.prepare("
            SELECT * From genres
            WHERE mov_name = ?1;
            ")?;
        let mut rows = stmt.query([&mov.name])?;
        while let Some(row) = rows.next()?{
            mov.genres.push(row.get(1)?)

        }
    }


    Ok(result)
}
























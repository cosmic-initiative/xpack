use std::{fs, io};
use std::io::Read;
use std::path::PathBuf;
use chumsky::primitive::Container;
use zip::ZipArchive;
use crate::dirs::cache_dir;
use crate::model::{CannonBall, CannonFile};
use crate::source::Source;
use crate::store::Store;

pub struct Cache {
  store: Store
}

impl Cache {

    pub fn new() -> Result<Self,anyhow::Error> {
        let store = Store::new()?;
        Ok(Cache{store})
    }

    pub fn fetch(&self, ball: &CannonBall ) -> Result<(),anyhow::Error> {
        let cache_dir= cache_dir()?;
        let mut ball_dir = cache_dir;
        ball_dir.push(ball.to_string());
        if !ball_dir.exists() {
println!("unzip dir: {}", ball_dir.to_str().unwrap());
            fs::create_dir_all(&ball_dir)?;
            let zip = self.store.fetch( &ball )?;
            let mut zip_path = ball_dir.clone();
            zip_path.push("zip");
println!("zip_path: {}", zip_path.to_str().unwrap());
            fs::write(zip_path.clone(), zip )?;
            let zip_file = fs::File::open(zip_path)?;
            let mut archive = ZipArchive::new(zip_file)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();

                if file.is_dir() {
                    let mut path = ball_dir.clone();
                    path.push("files");
                    path.push( file.name() );
println!("create dir: {}", path.to_str().unwrap());

                    fs::create_dir_all(path)?;
                } else {
                    let mut path = ball_dir.clone();
                    path.push("files");
                    path.push( file.name() );
println!("create file: {}", path.to_str().unwrap());
                    let mut out = fs::File::open(path)?;
                    io::copy(&mut file, &mut out).unwrap();
                }
            }
        } else {
            println!("dir exists");
        }

        Ok(())
    }


    pub fn get(&self, cannon_file: &CannonFile ) -> Result<Vec<u8>,anyhow::Error> {

        self.fetch(&cannon_file.ball)?;

//        Ok(fs::read(/file_path)?)
        unimplemented!()
    }

}
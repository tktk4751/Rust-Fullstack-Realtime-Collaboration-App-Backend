use actix_web::error::ErrorInternalServerError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn save_file(file: &[u8], path: &Path) -> Result<(), actix_web::Error> {
    let mut file = match fs::File::create(&path) {
        Err(why) => {
            return Err(ErrorInternalServerError(format!("Cannot create file: {:?}", why)));
        }
        Ok(file) => file,
    };

    match file.write_all(&file) {
        Err(why) => {
            return Err(ErrorInternalServerError(format!("Cannot write to file: {:?}", why)));
        }
        Ok(_) => Ok(()),
    }
}

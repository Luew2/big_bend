use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

use crate::user::User;

pub fn read_user_from_file(filename: &str) -> Result<User, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err("File does not exist.".into());
    }

    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let user: User = serde_json::from_str(&data)?;

    Ok(user)
}

pub fn write_user_to_file(filename: &str, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(user)?;
    let mut file = File::create(filename)?;
    
    file.write_all(data.as_bytes())?;

    Ok(())
}

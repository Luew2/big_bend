use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

use crate::user::User;

const USER_DATA_FILE: &'static str = "user_data.json";

pub fn load_user_data() -> Result<User, Box<dyn std::error::Error>> {
    let path = Path::new(USER_DATA_FILE);
    if !path.exists() {
        return Err("File does not exist.".into());
    }

    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let user: User = serder_json::from_str(&data)?;

    Ok(user)
}

pub fn save_user_data(user: &User) -> REsult<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(user)?;
    let mut file = File::create(USER_DATA_FILE)?;
    
    file.write_all(data.as_bytes())?;

    Ok(())
}




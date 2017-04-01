//! A collection of utils function for parol.rs.

use std::io::prelude::*;
use std::fs;
use std::error::Error;
use core::Parols;
use std::env;
use blowfish_ecb;

/// Constructs a Key from a String slice ( password ) and return it.
///
/// # Arguments
/// * `slice` - The slice containing the password ( min 4 and max 56 length ).
pub fn key_from_slice(slice: &str) -> Vec<u8> {
    if slice.len() < 4 {
        panic!("The slice length is less than 4.");
    }

    if slice.len() > 56 {
        panic!("The slice length is more than 56.");
    }

    return slice.as_bytes().to_vec()
}

/// Write a u8 array to a file. Return Ok(()) if succeed or a String with the Error if
///
/// # Arguments
/// * `path` - the filename.
/// * `buf` - the u8 array to write.
pub fn write_file(path: &str, buf: &[u8]) -> Result<(), String> {
    let mut file = match fs::File::create(path) {
        Ok(file) => file,
        Err(err) => panic!(err),
    };

    return match file.write_all(buf) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.description().to_string()),
    }
}

/// Read a file in an u8 array and return it or return a Err(String) if failed.
///
/// # Arguments
/// * `path` - the filename.
pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => panic!(err),
    };

    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => (),
        Err(err) => return Err(err.description().to_string()),
    }

    return Ok(data)
}

/// Load the database and return a ```Parols``` if succeed or a Err(String) if fail.
///
/// # Arguments
/// * `file`
/// * `password` - Password of the database.
pub fn load_database(password: &str) -> Result<Parols, String> {
    let database_crypted = match read_file(&database_file()) {
        Ok(database_crypted) => database_crypted,
        Err(err) => return Err(err),
    };

    let key = key_from_slice(password);

    let database_decrypted = blowfish_ecb::decrypt(&key, &database_crypted);

    let parols_json = match String::from_utf8(database_decrypted) {
        Ok(parols_json) => parols_json,
        Err(err) => return Err(err.description().to_string()),
    };

    return Ok(Parols::new_from_json(&parols_json));
}

/// Save and erase the parols, in the database. Return a Ok(()) if succeed or a Err(String) if fail.
///
/// # Arguments
/// * `parols` - The parols database.
/// * `password` - The password the database.
pub fn save_database(parols: &Parols, password: &str) -> Result<(), String> {
    let json = parols.to_json();
    let key = key_from_slice(password);
    let database_crypted = blowfish_ecb::encrypt(&key, json.as_bytes());

    match write_file(&database_file(), &database_crypted) {
        Ok(_) => {},
        Err(err) => return Err(err),
    }

    return Ok(())
}

/// Return a String containing the home directory.
pub fn home_dir() -> String {
    match env::home_dir() {
        Some(path) => {
            match path.to_str() {
                Some(home) => return String::from(home),
                None => panic!(),
            }
        },
        None => panic!(),
    }
}

/// Return a String containing the database directory.
pub fn database_dir() -> String {
    let home = home_dir();
    let database_dir = format!("{}/.config/parol", home);
    match fs::create_dir_all(&database_dir) {
        Ok(_) => return database_dir,
        Err(err) => panic!(err),
    }
}

/// Return a String containing the database filename.
pub fn database_file() -> String {
    let database_dir = database_dir();
    return format!("{}/parols.dbrs", database_dir);
}

pub fn nonce_file() -> String {
    let database_dir = database_dir();
    return format!("{}/parols.nonce", database_dir);
}
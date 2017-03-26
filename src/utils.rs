//! A collection of utils function for parol.rs.

use sodiumoxide::crypto::secretbox;
use std::io::prelude::*;
use std::fs;
use std::error::Error;
use core::Parols;
use std::env;

/// Constructs a Key from a String slice ( password ) and return it.
///
/// # Arguments
/// * `slice` - The slice containing the password ( max 32 length ).
pub fn key_from_slice(slice: &str) -> secretbox::xsalsa20poly1305::Key {
    if slice.len() > 32 {
        panic!("The slice length is more than 32.");
    }

    let mut array: [u8; 32] = [0; 32];

    for (index, byte) in slice.as_bytes().iter().enumerate() {
        array[index] = *byte;
    }

    return match secretbox::xsalsa20poly1305::Key::from_slice(&array) {
        Some(key) => key,
        None => panic!("Unknown error"),
    };
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
/// * `password` - Password of the database.
pub fn load_database(password: &str) -> Result<Parols, String> {
    let database_crypted = match read_file(&database_file()) {
        Ok(database_crypted) => database_crypted,
        Err(err) => return Err(err),
    };

    let nonce = match read_file("nonce") {
        Ok(nonce) => {
            match secretbox::xsalsa20poly1305::Nonce::from_slice(&nonce) {
                Some(nonce) => nonce,
                None => panic!("Cannot generate nonce !"),
            }
        },
        Err(err) => return Err(err),
    };

    let key = key_from_slice(password);

    let database_decrypted = match secretbox::open(&database_crypted, &nonce, &key) {
        Ok(database_decrypted) => database_decrypted,
        Err(err) => panic!(err),
    };

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
/// * `nonce` - A nonce (generated from `sodiumoxide::crypto::secretbox::gen_nonce()` or `utils::key_from_slice("password")`).
/// * `password` - The password the database.
pub fn save_database(parols: &Parols, nonce: &secretbox::xsalsa20poly1305::Nonce, password: &str) -> Result<(), String> {
    let json = parols.to_json();
    let key = key_from_slice(password);
    let crypted = secretbox::seal(&json.as_bytes(), &nonce, &key);
    let nonce = nonce.0;

    match write_file(&database_file(), &crypted) {
        Ok(_) => {},
        Err(err) => return Err(err),
    }

    match write_file("nonce", &nonce) {
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
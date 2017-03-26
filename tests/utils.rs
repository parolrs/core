extern crate parolrs;
extern crate sodiumoxide;

use parolrs::{utils, core};
use sodiumoxide::crypto::secretbox;

#[test]
fn test_key_from_slice() {
    let key = utils::key_from_slice("admin");

    assert_eq!(key.0.len(), 32);
}

#[test]
fn test_write_file() {
    let buf = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin vel.".as_bytes();
    let file = match utils::write_file("foo", &buf) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    };

    assert_eq!(file, Ok(()));
}

#[test]
fn test_read_file() {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin vel.";

    let buf = lorem.as_bytes();
    let file = match utils::write_file("foo", &buf) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    };

    assert_eq!(file, Ok(()));

    let data = match utils::read_file("foo") {
        Ok(d) => d,
        Err(_) => panic!(),
    };

    assert_eq!(String::from_utf8(data).unwrap(), lorem);
}

#[test]
fn test_database_dir() {
    assert_eq!(utils::database_dir(), format!("{}/.config/parol", utils::home_dir()));
}

#[test]
fn test_database_file() {
    assert_eq!(utils::database_file(), format!("{}/parols.dbrs", utils::database_dir()));
}

#[test]
fn test_database() {
    let mut parols = core::Parols::new();
    for _i in 0 .. 100 {
        let parol = core::Parol::new_with_arguments(
            "tox",
            "Ogromny",
            "admin",
            "blabla",
        );
        parols.push(parol);
    }

    let nonce = secretbox::gen_nonce();

    let password = "admin";

    match utils::save_database(&parols, &nonce, &password) {
        Ok(_) => {},
        Err(err) => panic!(err),
    }

    let parols2 = match utils::load_database("admin") {
        Ok(parols) => parols,
        Err(err) => panic!(err),
    };

    assert_eq!(parols.to_json(), parols2.to_json());
}
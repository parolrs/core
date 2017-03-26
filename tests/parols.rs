extern crate parolrs;

use parolrs::core::{Parol, Parols};
use parolrs::utils;

#[test]
fn test_json() {
    let mut parols = Parols::new();

    for _i in 0 .. 10 {
        let parol = Parol::new_with_arguments("tox", "Ogromny", "superstrongpassword", "");
        parols.push(parol);
    }

    let parols2 = Parols::new_from_json(&parols.to_json());

    // dirty
    for i in 0 .. 10 {
        let parols_i = match parols.get(i) {
            Some(parols) => parols,
            None => panic!("..."),
        };
        let parols2_i = match parols2.get(i) {
            Some(parols) => parols,
            None => panic!("..."),
        };

        assert_eq!(parols_i.get_application(), parols2_i.get_application());
        assert_eq!(parols_i.get_username(), parols2_i.get_username());
        assert_eq!(parols_i.get_password(), parols2_i.get_password());
        assert_eq!(parols_i.get_notes(), parols2_i.get_notes());
    }

    assert_eq!(parols.len(), parols2.len());
}

#[test]
fn test_load_and_save() {
    let parols1 = {
        let mut parols = Parols::new();

        for i in 0 .. 5 {
            let parol = Parol::new_with_arguments(
                &format!("tox{}", i),
                "Ogromny",
                "admin",
                &format!("{}", i * i * i),
            );

            parols.push(parol);
        }

        let parols_json = parols.to_json();

        match utils::write_file("database", parols_json.as_bytes()) {
            Ok(_) => println!("Database writed !"),
            Err(err) => panic!(err),
        }

        format!("{:?}", parols)
    };

    let parols2 = {
        let json = match utils::read_file("database") {
            Ok(json) => json,
            Err(err) => panic!(err),
        };

        let parols_json = match String::from_utf8(json) {
            Ok(parols_json) => parols_json,
            Err(err) => panic!(err),
        };

        let parols = Parols::new_from_json(&parols_json);

        println!("parols = {:#?}", parols);

        format!("{:?}", parols)
    };

    assert_eq!(parols1, parols2);
}
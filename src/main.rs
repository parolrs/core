extern crate parolrs;
extern crate sodiumoxide;

use parolrs::core::{Parols, Parol};
use parolrs::utils::{load_database, save_database};
use sodiumoxide::crypto::secretbox::gen_nonce;

// in 9.90user 3.55system 0:14.92elapsed 90%CPU :)
fn main() {
    let mut parols = Parols::new();
    for i in 0 .. 100_000 {
        let parol = Parol::new_with_arguments(
            &format!("tox{}", i),
            "Ogromny",
            "admin",
            "blabla",
        );
        parols.push(parol);
    }

    for i in 0 .. 100 {
        let nonce = gen_nonce();
        save_database(&parols, &nonce, "admin").unwrap();
        if i == 99 {
            println!("load_database(\"admin\").ok().unwrap() = {:#?}", load_database("admin").ok().unwrap());
        } else {
            load_database("admin").ok().unwrap();
        }
    }

}

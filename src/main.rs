extern crate parolrs;

use parolrs::core::{Parols, Parol};
use parolrs::utils::{load_database, save_database};

fn main() {
    let mut parols = Parols::new();
    for i in 0 .. 100_000 {
        parols.push(
            Parol::new_with_arguments(
                &format!("tox{}", i),
                "Ogromny",
                "admin",
                "blabla"
            )
        );
    }

    for _ in 0 .. 100 {
        save_database(&parols, "admin").unwrap();
        load_database("admin").ok().unwrap();
    }
}

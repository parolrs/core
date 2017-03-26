extern crate parolrs;

use parolrs::core::Parol;

#[test]
fn test_application() {
    let mut p = Parol::new();
    p.set_application("twitter");
    assert_eq!(p.get_application(), "twitter");
}

#[test]
fn test_username() {
    let mut p = Parol::new();
    p.set_username("Ogromny");
    assert_eq!(p.get_username(), "Ogromny");
}

#[test]
fn test_password() {
    let mut p = Parol::new();
    p.set_password("super_strong_password");
    assert_eq!(p.get_password(), "super_strong_password");
}

#[test]
fn test_notes() {
    let mut p = Parol::new();
    p.set_notes("Somes notes...");
    assert_eq!(p.get_notes(), "Somes notes...");
}

#[test]
fn test_new_with_arguments() {
    let p = Parol::new_with_arguments(
        "twitter",
        "Ogromny",
        "super_strong_password",
        "parol.rs rocks !",
    );

    assert_eq!(p.get_application(), "twitter");
    assert_eq!(p.get_username(), "Ogromny");
    assert_eq!(p.get_password(), "super_strong_password");
    assert_eq!(p.get_notes(), "parol.rs rocks !");
}
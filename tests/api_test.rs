use std::hash::{Hash, Hasher};

use stagiaire::Symbol;

#[test]
fn interned_string_has_same_value() {
    let sym = Symbol::new("foo");
    assert_eq!(sym.as_str(), "foo");
}

#[test]
fn same_strings_have_same_address() {
    let sym1 = Symbol::new("foo");
    let sym2 = Symbol::new("foo");
    assert_eq!(sym1.as_str().as_ptr(), sym2.as_str().as_ptr());
}

#[test]
fn different_strings_have_different_addresses() {
    let sym1 = Symbol::new("foo");
    let sym2 = Symbol::new("bar");
    assert_ne!(sym1.as_str().as_ptr(), sym2.as_str().as_ptr());
}

#[test]
fn a_copy_is_shallow() {
    let sym1 = Symbol::new("foo");
    let sym2 = sym1;
    assert_eq!(sym1.as_str().as_ptr(), sym2.as_str().as_ptr());
}

#[test]
fn hash_computed_on_string_address_not_value() {
    let sym = Symbol::new("zorglub");

    use std::collections::hash_map;
    let mut hasher_sym = hash_map::DefaultHasher::new();
    sym.hash(&mut hasher_sym);

    let mut hasher_str = hash_map::DefaultHasher::new();
    "zorglub".hash(&mut hasher_str);

    // This could fail because the string address and content could hash to the same value but
    // this seems unlikely.
    assert_ne!(hasher_sym.finish(), hasher_str.finish());
}

#[test]
fn compare_with_str_ref() {
    assert_eq!("foo", Symbol::new("foo"));
    assert_ne!("foo", Symbol::new("bar"));
    assert_eq!(Symbol::new("foo"), "foo");
    assert_ne!(Symbol::new("foo"), "bar");
}

#[test]
fn symbol_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Symbol>();
}

#[test]
fn symbol_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Symbol>();
}

#[test]
fn symbol_can_be_displayed() {
    let sym = Symbol::new("foo");
    assert_eq!(format!("{sym}"), "foo");
}

#[cfg(serde)]
#[test]
fn serialize() {
    let sym = Symbol::new("zorglub");
    let json = serde_json::to_string(&sym).unwrap();
    assert_eq!(json, r#""zorglub""#);
}

#[cfg(serde)]
#[test]
fn deserialize() {
    let sym = serde_json::from_str::<Symbol>(r#""zorglub""#).unwrap();
    assert_eq!(sym.as_str(), "zorglub")
}
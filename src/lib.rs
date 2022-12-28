//! A string interner.
//!
//! A string interner stores a pool of immutable strings keeping a single copy
//! of each string value.  A [`Symbol`] is a wrapper over a pointer to
//! one of these unique string values.  Symbols can be compared quickly (pointer
//! rather than string comparisons) and are cheaper to store than strings when
//! several occurrences of a given string exist.
//!
//! # Examples
//!
//! ```
//! use stagiaire::Symbol;
//!
//! // Create a new symbol.
//! let a_foo = Symbol::new("foo");
//! assert_eq!(a_foo.as_str(), "foo");
//!
//! // Create another symbol that refers to an existing value.
//! let another_foo = Symbol::new("foo");
//! assert_eq!(a_foo, another_foo);
//!
//! // Both symbols point to the same underlying value.
//! assert_eq!(a_foo.as_str().as_ptr(), another_foo.as_str().as_ptr());
//!
//! // A symbol has the same size as a reference.
//! assert_eq!(std::mem::size_of::<Symbol>(), std::mem::size_of::<&str>());
//!
//! // Symbols pointing to different values are not equal.
//! let a_bar = Symbol::new("bar");
//! assert_ne!(a_bar, a_foo);
//! ```
//!
//! # Lifetime
//!
//! The interner is a process-wide singleton not exposed programmatically and
//! string values stored there persist until the owning process terminates and
//! have therefore a `'static` lifetime.
//!
//! # Thread-safety
//!
//! [`Symbol`] values can be created and accessed from multiple threads.
//!
//! [`Symbol`]: struct.Symbol.html

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use lazy_static::lazy_static;

/// Wrapper over a reference to an interned string.
///
/// See crate-level documentation for example and details.
#[derive(Debug, Clone, Copy)]
pub struct Symbol {
    inner: &'static str,
}

impl Symbol {
    /// Inserts in the pool the value `s` if it is no already there and returns
    /// a symbol pointing to this new value or the existing one.
    pub fn new<R: AsRef<str>>(s: R) -> Symbol {
        Symbol {
            inner: intern(s.as_ref()),
        }
    }

    /// Returns a reference to the string pointed to by this symbol.
    pub fn as_str(&self) -> &'static str {
        self.inner
    }
}

impl From<&str> for Symbol {
    /// Generates a symbol for `source`.
    fn from(source: &str) -> Self {
        Symbol::new(source)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.inner.as_ptr() == other.inner.as_ptr()
    }
}

impl Eq for Symbol {}

// Implement mixed comparisons.
// Code lifted from String implementation.
// I do not understand why &'a str versions are required nor how they work.
// I naively thought str versions would be sufficient.

impl PartialEq<str> for Symbol {
    fn eq(&self, other: &str) -> bool {
        self.inner[..] == other[..]
    }
}

impl PartialEq<Symbol> for str {
    fn eq(&self, other: &Symbol) -> bool {
        self[..] == other.inner[..]
    }
}

impl<'a> PartialEq<Symbol> for &'a str {
    fn eq(&self, other: &Symbol) -> bool {
        self[..] == other.inner[..]
    }
}

impl<'a> PartialEq<&'a str> for Symbol {
    fn eq(&self, other: &&'a str) -> bool {
        self.inner[..] == other[..]
    }
}

impl Hash for Symbol {
    /// Returns a hash of the pointer wrapped by this symbol (rather than the
    /// pointed-to string content).
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.as_ptr().hash(state);
    }
}

lazy_static! {
    // All strings interned so far.
    static ref STRINGS : Mutex<HashSet<&'static str>> = {
        Mutex::new(HashSet::new())
    };
}

// Returns a reference to a string that has the same value as `s` and is guaranteed to be unique.
fn intern(str: &str) -> &'static str {
    let mut g = STRINGS.lock().unwrap();
    // TODO: Use HashSet::get_or_insert() when stabilized
    match g.get(str) {
        Some(s) => s,
        None => {
            let b = Box::new(str.to_string());
            let s = Box::leak(b).as_str();
            g.insert(s);
            s
        }
    }
}

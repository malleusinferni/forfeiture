extern crate rayon_core;

use std::fmt::Debug;
use std::hash::Hash;
//use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Error;

pub type Result<T, E=Error> = std::result::Result<T, E>;

pub trait Id: 'static + Clone + Debug + Eq + Hash {
    type Target: Asset;

    fn load(self) -> Result<Self::Target>;
}

pub trait Asset: Default {
    // Empty marker trait
    // `Default` is required for the creation of placeholders
}

#[cfg(test)]
mod example {
    use super::*;

    use std::fs::File;
    use std::io::{self, Read};
    use std::path::PathBuf;

    impl Id for PathBuf {
        type Target = String;

        fn load(self) -> Result<String> {
            let mut buf = String::default();
            File::open(&self)?.read_to_string(&mut buf)?;
            Ok(buf)
        }
    }

    impl Asset for String {
    }

    impl From<io::Error> for Error {
        fn from(_err: io::Error) -> Self {
            Error
        }
    }

    #[test]
    fn load_a_string() {
        let path = PathBuf::from("README.md");
        let _asset = Id::load(path).unwrap();
    }
}

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Clone, PartialEq, Eq, Deserialize)]
pub struct Version(u8);

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Version {
    pub fn new(version: u8) -> Version {
        Version(version)
    }

    pub fn parse(&self) -> String {
        format!("{:08b}", (self.0 << 6) | 0)
    }

    pub fn from_vec(version: &[u8]) -> Version {
        Version(version[0])
    }
}
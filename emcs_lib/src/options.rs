use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::mc_versions::MCVersions;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Options {
    pub mcver: MCVersions,
    dir: String,
}

impl Options {
    pub fn dir(&self) -> &Path {
        Path::new(&self.dir)
    }

    pub fn new(mcver: MCVersions, dir: String) -> Self {
        Self { mcver, dir }
    }
}

use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SemVer {
    major: u64,
    minor: u64,
    patch: u64,
}

impl SemVer {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn new_with_string(version: &str) -> Result<Self> {
        let mut version = version.split('.');
        let major = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No major version found"))?
            .parse()?;
        let minor = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No minor version found"))?
            .parse()?;
        let patch = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No patch version found"))?
            .parse()?;
        Ok(Self::new(major, minor, patch))
    }

    pub fn next_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }

    pub fn next_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }

    pub fn next_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        Ok(())
    }
}

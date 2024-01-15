use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Profile {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "dev")]
    Dev,
    #[serde(rename = "prod")]
    Prod,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Profile::Test => write!(f, "test"),
            Profile::Dev => write!(f, "dev"),
            Profile::Prod => write!(f, "prod"),
        }
    }
}

impl TryFrom<&str> for Profile {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{other} is not a supported environment. Use either `dev` or `prod` or `test`."
            )),
        }
    }
}

impl TryFrom<String> for Profile {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Profile::try_from(&*value)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::AppConfig;

    pub use super::*;

    #[test]
    pub fn test_read_app_config() {
        let _config = AppConfig::read().unwrap();
    }

    #[test]
    pub fn test_profile_to_string() {
        let profile: Profile = Profile::try_from("Dev").unwrap();
        assert_eq!(profile, Profile::Dev)
    }
}

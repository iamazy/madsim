//! Simulation configuration.

use std::{hash::Hash, str::FromStr};

use crate::net::{self, tcp};
use serde::{Deserialize, Serialize};

/// Simulation configuration.
#[cfg_attr(docsrs, doc(cfg(madsim)))]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Hash, Clone)]
pub struct Config {
    /// Network configurations.
    #[serde(default)]
    pub net: net::Config,

    /// Tcp Configurations
    #[serde(default)]
    pub tcp: tcp::TcpConfig,
}

impl Config {
    /// Returns the hash value of this config.
    pub fn hash(&self) -> u64 {
        ahash::RandomState::with_seed(0).hash_one(self)
    }
}

/// Parse a config from TOML.
impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

/// Print the config into TOML.
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string_pretty(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn parse() {
        // TODO: better way to parse Duration
        let config: Config = r#"
        [net]
        packet_loss_rate = 0.1
        send_latency = { start = { secs = 0, nanos = 1000000 }, end = { secs = 0, nanos = 10000000 } }
        
        [tcp]
        "#
        .parse()
        .unwrap();
        assert_eq!(
            config,
            Config {
                net: net::Config {
                    packet_loss_rate: 0.1,
                    send_latency: Duration::from_millis(1)..Duration::from_millis(10)
                },
                tcp: tcp::TcpConfig {}
            }
        );
    }
}

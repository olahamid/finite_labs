//! Typed view of `data/site.toml`. If the file doesn't match these structs,
//! the server refuses to start, so a typo can never reach production.

use std::{fmt, fs, io, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteConfig {
    pub name: String,
    pub domain: String,
    pub description: String,
    pub contact_email: String,
    pub nav: Vec<Link>,
    pub cta: Link,
    pub hero: Hero,
    pub sections: Vec<Section>,
    pub footer: Footer,
    pub about: About,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub label: String,
    pub href: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hero {
    pub eyebrow: String,
    pub headline: String,
    pub subline: String,
    pub primary: Link,
    pub secondary: Link,
}

/// One homepage block ("We teach", "We publish our work", ...).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// Used as the HTML `id`, so nav links like `/#teach` can jump to it.
    pub id: String,
    pub eyebrow: String,
    pub heading: String,
    pub body: String,
    /// Optional call to action under the section text.
    #[serde(default)]
    pub link: Option<Link>,
    #[serde(default)]
    pub cards: Vec<Card>,
    /// Groups nested inside this section, e.g. the Playground inside Learn.
    #[serde(default)]
    pub subsections: Vec<Subsection>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subsection {
    /// HTML `id`, so links like `/#playground` still jump straight to it.
    pub id: String,
    pub heading: String,
    pub body: String,
    #[serde(default)]
    pub cards: Vec<Card>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub title: String,
    pub summary: String,
    /// Small badge such as "Live" or "Coming soon".
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub href: Option<String>,
}

impl Card {
    pub fn is_live(&self) -> bool {
        self.label.as_deref() == Some("Live")
    }

    /// First 8 hex characters of SHA-256(title): a real value used as quiet
    /// decorative texture on cards. Plain hex, no `0x` (not a crypto-coin look).
    pub fn fingerprint(&self) -> String {
        let mut hex = crypto::hash::sha256_hex(self.title.as_bytes());
        hex.truncate(8);
        hex
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Footer {
    pub blurb: String,
    pub columns: Vec<FooterColumn>,
    #[serde(default)]
    pub social: Vec<Link>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FooterColumn {
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct About {
    pub heading: String,
    pub paragraphs: Vec<String>,
}

/// Why loading the config failed. Each variant keeps the original error so
/// the message tells you exactly what to fix.
#[derive(Debug)]
pub enum ConfigError {
    Read(io::Error),
    Parse(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Read(e) => write!(f, "could not read site config: {e}"),
            ConfigError::Parse(e) => write!(f, "site config is invalid: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl SiteConfig {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let raw = fs::read_to_string(path).map_err(ConfigError::Read)?;
        toml::from_str(&raw).map_err(ConfigError::Parse)
    }
}

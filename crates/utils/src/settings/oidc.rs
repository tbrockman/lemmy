use core::fmt;
use doku::Document;
use openidconnect::core::CoreProviderMetadata;
use serde::{
  de::{self, Visitor},
  Deserialize,
  Deserializer,
  Serialize,
  Serializer,
};
use smart_default::SmartDefault;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(deny_unknown_fields)]
pub struct OIDCConfig {
  providers: Vec<OIDCProvider>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Document)]
pub struct OIDCProvider {
  pub client_id: String,
  pub client_secret: String,
  pub url: Url,
  pub name: ProviderName,
  #[serde(skip)]
  pub metadata: Option<CoreProviderMetadata>,
}

#[derive(Clone, Debug, PartialEq, Document)]
pub enum ProviderName {
  Google,
  Slack,
  Facebook,
  Apple,
  Okta,
  Microsoft,
  Other(String),
}

impl FromStr for ProviderName {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "google" => Ok(ProviderName::Google),
      "slack" => Ok(ProviderName::Slack),
      "facebook" => Ok(ProviderName::Facebook),
      "apple" => Ok(ProviderName::Apple),
      "okta" => Ok(ProviderName::Okta),
      "microsoft" => Ok(ProviderName::Microsoft),
      other => Ok(ProviderName::Other(other.to_owned())),
    }
  }
}

impl Serialize for ProviderName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match *self {
      ProviderName::Google => serializer.serialize_str("google"),
      ProviderName::Slack => serializer.serialize_str("slack"),
      ProviderName::Facebook => serializer.serialize_str("facebook"),
      ProviderName::Apple => serializer.serialize_str("apple"),
      ProviderName::Okta => serializer.serialize_str("okta"),
      ProviderName::Microsoft => serializer.serialize_str("microsoft"),
      ProviderName::Other(ref s) => serializer.serialize_str(s),
    }
  }
}

impl<'de> Deserialize<'de> for ProviderName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ProviderNameVisitor;

    impl<'de> Visitor<'de> for ProviderNameVisitor {
      type Value = ProviderName;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a provider name")
      }

      fn visit_str<E>(self, value: &str) -> Result<ProviderName, E>
      where
        E: de::Error,
      {
        ProviderName::from_str(value).map_err(de::Error::custom)
      }
    }

    deserializer.deserialize_str(ProviderNameVisitor)
  }
}

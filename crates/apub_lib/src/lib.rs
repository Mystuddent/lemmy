use activitystreams::{
  base::AnyBase,
  error::DomainError,
  primitives::OneOrMany,
  unparsed::Unparsed,
};
pub use lemmy_apub_lib_derive::*;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

pub mod webfinger;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum PublicUrl {
  #[serde(rename = "https://www.w3.org/ns/activitystreams#Public")]
  Public,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCommonFields {
  #[serde(rename = "@context")]
  pub context: OneOrMany<AnyBase>,
  id: Url,
  pub actor: Url,

  // unparsed fields
  #[serde(flatten)]
  pub unparsed: Unparsed,
}

impl ActivityCommonFields {
  pub fn id_unchecked(&self) -> &Url {
    &self.id
  }
}

#[async_trait::async_trait(?Send)]
pub trait ActivityHandler {
  async fn verify(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError>;

  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError>;
  fn common(&self) -> &ActivityCommonFields;
}

pub fn verify_domains_match(a: &Url, b: &Url) -> Result<(), LemmyError> {
  if a.domain() != b.domain() {
    return Err(DomainError.into());
  }
  Ok(())
}

pub fn verify_domains_match_opt(a: &Url, b: Option<&Url>) -> Result<(), LemmyError> {
  if let Some(b2) = b {
    return verify_domains_match(a, b2);
  }
  Ok(())
}

pub fn verify_urls_match(a: &Url, b: &Url) -> Result<(), LemmyError> {
  if a != b {
    return Err(DomainError.into());
  }
  Ok(())
}

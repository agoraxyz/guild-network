use super::{Identity, IdentityWithAuth};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Platform {
    EvmChain,
    Discord,
    Telegram,
}

/// Stores the user's identities in a HashMap that allows
/// `O(1)` access to platform-specific identities.
pub struct IdentityMap(HashMap<Platform, Identity>);

impl IdentityMap {
    pub fn from_verified_identities(
        ids: Vec<IdentityWithAuth>,
        verification_msg: &str,
    ) -> Result<Self, anyhow::Error> {
        let map = ids
            .into_iter()
            .map(|id| {
                id.verify(verification_msg)?;
                Ok::<(Platform, Identity), anyhow::Error>(id.into_platform_with_id())
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(map))
    }

    pub fn into_identity_vec(self) -> Vec<Identity> {
        self.0.into_values().collect()
    }

    pub fn inner(&self) -> &HashMap<Platform, Identity> {
        &self.0
    }
}

use dpp::{identity::accessors::IdentitySettersV0, prelude::Identity};
use napi_derive::napi;

use crate::{
    dynamic_value::{TryToU64, Uint64String},
    enums::platform_version::PlatformVersionWASM,
    identifier::IdentifierWASM,
};

#[derive(Clone)]
#[napi(js_name = IdentityWASM)]
pub struct IdentityWASM {
    identity: Identity,
}

impl From<Identity> for IdentityWASM {
    fn from(identity: Identity) -> Self {
        IdentityWASM { identity }
    }
}

impl From<IdentityWASM> for Identity {
    fn from(identity: IdentityWASM) -> Self {
        identity.identity
    }
}

#[napi]
impl IdentityWASM {
    #[napi(constructor)]
    pub fn new(
        id: &IdentifierWASM,
        platform_version: PlatformVersionWASM,
    ) -> Result<Self, napi::Error> {
        Ok(IdentityWASM {
            identity: Identity::create_basic_identity(id.clone().into(), &platform_version.into())
                .map_err(|err| napi::Error::new(napi::Status::GenericFailure, err.to_string()))?,
        })
    }

    #[napi(setter, js_name = "id")]
    pub fn set_id(&mut self, id: &IdentifierWASM) {
        self.identity.set_id(id.clone().into());
    }

    #[napi(setter, js_name = "balance")]
    pub fn set_balance(&mut self, balance: Uint64String) -> Result<(), napi::Error> {
        self.identity.set_balance(balance.try_to_u64()?);
        Ok(())
    }

    #[napi(setter, js_name = "revision")]
    pub fn set_revision(&mut self, revision: Uint64String) -> Result<(), napi::Error> {
        self.identity.set_revision(revision.try_to_u64()?);
        Ok(())
    }
}

use dpp::dashcore::secp256k1::hashes::hex::{Case, DisplayHex};
use dpp::platform_value::string_encoding::{decode, encode};
use dpp::serialization::{PlatformDeserializable, PlatformSerializable};
use dpp::{
    dashcore::Network,
    identity::{
        KeyType, Purpose, SecurityLevel,
        hash::IdentityPublicKeyHashMethodsV0,
        identity_public_key::{
            accessors::v0::{IdentityPublicKeyGettersV0, IdentityPublicKeySettersV0},
            v0::IdentityPublicKeyV0,
        },
    },
    platform_value::{BinaryData, string_encoding::Encoding},
    prelude::IdentityPublicKey,
};
use napi::Status;
use napi_derive::napi;

use crate::{
    dynamic_value::{DynamicValue, TryToU64, Uint64String},
    enums::{
        key_type::KeyTypeWASM, network::NetworkWASM, purpose::PurposeWASM,
        security_level::SecurityLevelWASM,
    },
    utils::WithJsError,
};

#[derive(Clone)]
#[napi(js_name = "IdentityPublicKeyWASM")]
pub struct IdentityPublicKeyWASM {
    public_key: IdentityPublicKey,
}

impl From<IdentityPublicKey> for IdentityPublicKeyWASM {
    fn from(public_key: IdentityPublicKey) -> Self {
        IdentityPublicKeyWASM { public_key }
    }
}

impl From<IdentityPublicKeyWASM> for IdentityPublicKey {
    fn from(public_key: IdentityPublicKeyWASM) -> Self {
        public_key.public_key
    }
}

#[napi]
impl IdentityPublicKeyWASM {
    #[napi(constructor)]
    pub fn new(
        id: u32,
        js_purpose: DynamicValue,
        js_security_level: DynamicValue,
        js_key_type: DynamicValue,
        read_only: bool,
        binary_data: String,
        js_disabled_at: Option<Uint64String>,
        // TODO: Implement js_contract_bounds
    ) -> Result<Self, napi::Error> {
        let purpose: PurposeWASM = js_purpose.try_into()?;
        let security_level: SecurityLevelWASM = js_security_level.try_into()?;
        let key_type: KeyTypeWASM = js_key_type.try_into()?;
        let disabled_at = js_disabled_at.map(|val| val.try_to_u64()).transpose()?;

        Ok(IdentityPublicKeyWASM {
            public_key: IdentityPublicKey::from(IdentityPublicKeyV0 {
                id,
                purpose: purpose.into(),
                security_level: security_level.into(),
                contract_bounds: None,
                key_type: key_type.into(),
                read_only,
                data: BinaryData::from_string(binary_data.as_str(), Encoding::Hex)
                    .map_err(|err| napi::Error::new(Status::GenericFailure, err.to_string()))?,
                disabled_at,
            }),
        })
    }

    #[napi(js_name = "validatePrivateKey")]
    pub fn validate_private_key(
        &self,
        js_private_key_bytes: Vec<u8>,
        js_network: DynamicValue,
    ) -> Result<bool, napi::Error> {
        let mut private_key_bytes = [0u8; 32];
        let len = js_private_key_bytes.len().min(32);
        private_key_bytes[..len].copy_from_slice(&js_private_key_bytes[..len]);

        let network = Network::from(NetworkWASM::try_from(js_network)?);

        self.public_key
            .validate_private_key_bytes(&private_key_bytes, network)
            .with_js_error()
    }

    #[napi(getter, js_name = "keyId")]
    pub fn get_key_id(&self) -> u32 {
        self.public_key.id()
    }

    #[napi(getter, js_name = purpose)]
    pub fn get_purpose(&self) -> String {
        PurposeWASM::from(self.public_key.purpose()).into()
    }

    #[napi(getter, js_name = purposeNumber)]
    pub fn get_purpose_number(&self) -> PurposeWASM {
        PurposeWASM::from(self.public_key.purpose())
    }

    #[napi(getter, js_name = securityLevel)]
    pub fn get_security_level(&self) -> String {
        SecurityLevelWASM::from(self.public_key.security_level()).into()
    }

    #[napi(getter, js_name = securityLevelNumber)]
    pub fn get_security_level_number(&self) -> SecurityLevelWASM {
        SecurityLevelWASM::from(self.public_key.security_level())
    }

    #[napi(getter, js_name = keyType)]
    pub fn get_key_type(&self) -> String {
        KeyTypeWASM::from(self.public_key.key_type()).into()
    }

    #[napi(getter, js_name = keyTypeNumber)]
    pub fn get_key_type_number(&self) -> KeyTypeWASM {
        KeyTypeWASM::from(self.public_key.key_type())
    }

    #[napi(getter, js_name = readOnly)]
    pub fn get_read_only(&self) -> bool {
        self.public_key.read_only()
    }

    #[napi(getter, js_name = data)]
    pub fn get_data(&self) -> String {
        self.public_key.data().to_string(Encoding::Hex)
    }

    #[napi(getter, js_name = disabledAt)]
    pub fn get_disabled_at(&self) -> Option<Uint64String> {
        self.public_key.disabled_at().map(|num| num.into())
    }

    #[napi(setter, js_name = keyId)]
    pub fn set_key_id(&mut self, key_id: u32) {
        self.public_key.set_id(key_id)
    }

    #[napi(setter, js_name = purpose)]
    pub fn set_purpose(&mut self, purpose: DynamicValue) -> Result<(), napi::Error> {
        Ok(self
            .public_key
            .set_purpose(Purpose::from(PurposeWASM::try_from(purpose)?)))
    }

    #[napi(setter, js_name = purposeNumber)]
    pub fn set_purpose_number(&mut self, purpose: DynamicValue) -> Result<(), napi::Error> {
        self.set_purpose(purpose)
    }

    #[napi(setter, js_name = securityLevel)]
    pub fn set_security_level(&mut self, security_level: DynamicValue) -> Result<(), napi::Error> {
        Ok(self
            .public_key
            .set_security_level(SecurityLevel::from(SecurityLevelWASM::try_from(
                security_level,
            )?)))
    }

    #[napi(setter, js_name = securityLevelNumber)]
    pub fn set_security_level_number(
        &mut self,
        security_level: DynamicValue,
    ) -> Result<(), napi::Error> {
        self.set_security_level(security_level)
    }

    #[napi(setter, js_name = keyType)]
    pub fn set_key_type(&mut self, key_type: DynamicValue) -> Result<(), napi::Error> {
        Ok(self
            .public_key
            .set_key_type(KeyType::from(KeyTypeWASM::try_from(key_type)?)))
    }

    #[napi(setter, js_name = keyTypeNumber)]
    pub fn set_key_type_number(&mut self, key_type: DynamicValue) -> Result<(), napi::Error> {
        self.set_key_type(key_type)
    }

    #[napi(setter, js_name = readOnly)]
    pub fn set_read_only(&mut self, read_only: bool) {
        self.public_key.set_read_only(read_only)
    }

    #[napi(setter, js_name = data)]
    pub fn set_data(&mut self, binary_data: String) -> Result<(), napi::Error> {
        let data = BinaryData::from_string(binary_data.as_str(), Encoding::Hex)
            .map_err(|err| napi::Error::new(Status::GenericFailure, err.to_string()))?;

        Ok(self.public_key.set_data(data))
    }

    #[napi(setter, js_name = disabledAt)]
    pub fn set_disabled_at(&mut self, disabled_at: Uint64String) -> Result<(), napi::Error> {
        self.public_key.set_disabled_at(disabled_at.try_to_u64()?);
        Ok(())
    }

    #[napi(js_name = removeDisabledAt)]
    pub fn remove_disabled_at(&mut self) {
        self.public_key.remove_disabled_at()
    }

    #[napi(js_name = "getPublicKeyHash")]
    pub fn public_key_hash(&self) -> Result<String, napi::Error> {
        let hash = self
            .public_key
            .public_key_hash()
            .with_js_error()
            .map(|slice| slice.to_vec())?
            .to_hex_string(Case::Lower);

        Ok(hash)
    }

    #[napi(js_name = "isMaster")]
    pub fn is_master(&self) -> bool {
        self.public_key.is_master()
    }

    #[napi(js_name = bytes)]
    pub fn to_byes(&self) -> Result<Vec<u8>, napi::Error> {
        self.public_key.serialize_to_bytes().with_js_error()
    }

    #[napi(js_name = hex)]
    pub fn to_hex(&self) -> Result<String, napi::Error> {
        Ok(encode(
            self.public_key
                .serialize_to_bytes()
                .with_js_error()?
                .as_slice(),
            Encoding::Hex,
        ))
    }

    #[napi(js_name = base64)]
    pub fn to_base64(&self) -> Result<String, napi::Error> {
        Ok(encode(
            self.public_key
                .serialize_to_bytes()
                .with_js_error()?
                .as_slice(),
            Encoding::Base64,
        ))
    }

    #[napi(js_name = fromBytes)]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<IdentityPublicKeyWASM, napi::Error> {
        let public_key =
            IdentityPublicKey::deserialize_from_bytes(bytes.as_slice()).with_js_error()?;

        Ok(IdentityPublicKeyWASM { public_key })
    }

    #[napi(js_name = fromHex)]
    pub fn from_hex(hex: String) -> Result<IdentityPublicKeyWASM, napi::Error> {
        let bytes = decode(&hex, Encoding::Hex)
            .map_err(|err| napi::Error::new(Status::GenericFailure, err.to_string()))?;

        let public_key =
            IdentityPublicKey::deserialize_from_bytes(bytes.as_slice()).with_js_error()?;

        Ok(IdentityPublicKeyWASM { public_key })
    }

    #[napi(js_name = fromBase64)]
    pub fn from_base64(hex: String) -> Result<IdentityPublicKeyWASM, napi::Error> {
        let bytes = decode(&hex, Encoding::Base64)
            .map_err(|err| napi::Error::new(Status::GenericFailure, err.to_string()))?;

        let public_key =
            IdentityPublicKey::deserialize_from_bytes(bytes.as_slice()).with_js_error()?;

        Ok(IdentityPublicKeyWASM { public_key })
    }
}

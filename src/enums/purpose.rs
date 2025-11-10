use dpp::identity::Purpose;
use napi::Status;
use napi_derive::napi;

use crate::dynamic_value::{DynamicValue, TryToU64};

#[napi(js_name = "PurposeWASM")]
pub enum PurposeWASM {
    AUTHENTICATION = 0,
    ENCRYPTION = 1,
    DECRYPTION = 2,
    TRANSFER = 3,
    SYSTEM = 4,
    VOTING = 5,
    OWNER = 6,
}

impl From<Purpose> for PurposeWASM {
    fn from(value: Purpose) -> Self {
        match value {
            Purpose::AUTHENTICATION => PurposeWASM::AUTHENTICATION,
            Purpose::ENCRYPTION => PurposeWASM::ENCRYPTION,
            Purpose::DECRYPTION => PurposeWASM::DECRYPTION,
            Purpose::TRANSFER => PurposeWASM::TRANSFER,
            Purpose::SYSTEM => PurposeWASM::SYSTEM,
            Purpose::VOTING => PurposeWASM::VOTING,
            Purpose::OWNER => PurposeWASM::OWNER,
        }
    }
}

impl From<PurposeWASM> for Purpose {
    fn from(purpose: PurposeWASM) -> Self {
        match purpose {
            PurposeWASM::AUTHENTICATION => Purpose::AUTHENTICATION,
            PurposeWASM::ENCRYPTION => Purpose::ENCRYPTION,
            PurposeWASM::DECRYPTION => Purpose::DECRYPTION,
            PurposeWASM::TRANSFER => Purpose::TRANSFER,
            PurposeWASM::SYSTEM => Purpose::SYSTEM,
            PurposeWASM::VOTING => Purpose::VOTING,
            PurposeWASM::OWNER => Purpose::OWNER,
        }
    }
}

impl From<PurposeWASM> for String {
    fn from(value: PurposeWASM) -> Self {
        match value {
            PurposeWASM::AUTHENTICATION => String::from("AUTHENTICATION"),
            PurposeWASM::ENCRYPTION => String::from("ENCRYPTION"),
            PurposeWASM::DECRYPTION => String::from("DECRYPTION"),
            PurposeWASM::TRANSFER => String::from("TRANSFER"),
            PurposeWASM::SYSTEM => String::from("SYSTEM"),
            PurposeWASM::VOTING => String::from("VOTING"),
            PurposeWASM::OWNER => String::from("OWNER"),
        }
    }
}

impl TryFrom<u8> for PurposeWASM {
    type Error = napi::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PurposeWASM::AUTHENTICATION),
            1 => Ok(PurposeWASM::ENCRYPTION),
            2 => Ok(PurposeWASM::DECRYPTION),
            3 => Ok(PurposeWASM::TRANSFER),
            4 => Ok(PurposeWASM::SYSTEM),
            5 => Ok(PurposeWASM::VOTING),
            6 => Ok(PurposeWASM::OWNER),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid purpose value",
            )),
        }
    }
}

impl TryFrom<String> for PurposeWASM {
    type Error = napi::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "authentication" => Ok(PurposeWASM::AUTHENTICATION),
            "encryption" => Ok(PurposeWASM::ENCRYPTION),
            "decryption" => Ok(PurposeWASM::DECRYPTION),
            "transfer" => Ok(PurposeWASM::TRANSFER),
            "system" => Ok(PurposeWASM::SYSTEM),
            "voting" => Ok(PurposeWASM::VOTING),
            "owner" => Ok(PurposeWASM::OWNER),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid purpose value",
            )),
        }
    }
}

impl TryFrom<DynamicValue> for PurposeWASM {
    type Error = napi::Error;

    fn try_from(value: DynamicValue) -> Result<Self, Self::Error> {
        match value {
            DynamicValue::Text(str) => PurposeWASM::try_from(str),
            DynamicValue::Uint8(num) => PurposeWASM::try_from(num),
            DynamicValue::Uint16(num) => PurposeWASM::try_from(num as u8),
            DynamicValue::Uint32(num) => PurposeWASM::try_from(num as u8),
            DynamicValue::Uint64(num_str) => PurposeWASM::try_from(num_str.try_to_u64()? as u8),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid purpose value",
            )),
        }
    }
}

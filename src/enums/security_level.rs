use dpp::identity::SecurityLevel;
use napi::Status;
use napi_derive::napi;

use crate::dynamic_value::{DynamicValue, TryToU64};

#[napi(js_name="SecurityLevel")]
pub enum SecurityLevelWASM {
    MASTER = 0,
    CRITICAL = 1,
    HIGH = 2,
    MEDIUM = 3,
}

impl From<SecurityLevelWASM> for String {
    fn from(level: SecurityLevelWASM) -> String {
        match level {
            SecurityLevelWASM::MASTER => String::from("MASTER"),
            SecurityLevelWASM::CRITICAL => String::from("CRITICAL"),
            SecurityLevelWASM::HIGH => String::from("HIGH"),
            SecurityLevelWASM::MEDIUM => String::from("MEDIUM"),
        }
    }
}

impl From<SecurityLevelWASM> for SecurityLevel {
    fn from(security_level: SecurityLevelWASM) -> Self {
        match security_level {
            SecurityLevelWASM::MASTER => SecurityLevel::MASTER,
            SecurityLevelWASM::CRITICAL => SecurityLevel::CRITICAL,
            SecurityLevelWASM::HIGH => SecurityLevel::HIGH,
            SecurityLevelWASM::MEDIUM => SecurityLevel::MEDIUM,
        }
    }
}

impl From<SecurityLevel> for SecurityLevelWASM {
    fn from(security_level: SecurityLevel) -> Self {
        match security_level {
            SecurityLevel::MASTER => SecurityLevelWASM::MASTER,
            SecurityLevel::CRITICAL => SecurityLevelWASM::CRITICAL,
            SecurityLevel::HIGH => SecurityLevelWASM::HIGH,
            SecurityLevel::MEDIUM => SecurityLevelWASM::MEDIUM,
        }
    }
}

impl TryFrom<u8> for SecurityLevelWASM {
    type Error = napi::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SecurityLevelWASM::MASTER),
            1 => Ok(SecurityLevelWASM::CRITICAL),
            2 => Ok(SecurityLevelWASM::HIGH),
            3 => Ok(SecurityLevelWASM::MEDIUM),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid security level value",
            )),
        }
    }
}

impl TryFrom<String> for SecurityLevelWASM {
    type Error = napi::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "master" => Ok(SecurityLevelWASM::MASTER),
            "critical" => Ok(SecurityLevelWASM::CRITICAL),
            "high" => Ok(SecurityLevelWASM::HIGH),
            "medium" => Ok(SecurityLevelWASM::MEDIUM),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid security level value",
            )),
        }
    }
}

impl TryFrom<DynamicValue> for SecurityLevelWASM {
    type Error = napi::Error;

    fn try_from(value: DynamicValue) -> Result<Self, Self::Error> {
        match value {
            DynamicValue::Text(str) => SecurityLevelWASM::try_from(str),
            DynamicValue::Uint8(num) => SecurityLevelWASM::try_from(num),
            DynamicValue::Uint16(num) => SecurityLevelWASM::try_from(num as u8),
            DynamicValue::Uint32(num) => SecurityLevelWASM::try_from(num as u8),
            DynamicValue::Uint64(num_str) => {
                SecurityLevelWASM::try_from(num_str.try_to_u64()? as u8)
            }
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid security level value",
            )),
        }
    }
}

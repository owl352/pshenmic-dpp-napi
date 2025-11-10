use dpp::dashcore::Network;
use napi::Status;
use napi_derive::napi;

use crate::dynamic_value::{DynamicValue, TryToU64};

#[napi]
#[allow(non_camel_case_types)]
pub enum NetworkWASM {
    Mainnet = 0,
    Testnet = 1,
    Devnet = 2,
    Regtest = 3,
}

impl From<NetworkWASM> for String {
    fn from(value: NetworkWASM) -> Self {
        match value {
            NetworkWASM::Mainnet => "Mainnet".to_string(),
            NetworkWASM::Testnet => "Testnet".to_string(),
            NetworkWASM::Devnet => "Devnet".to_string(),
            NetworkWASM::Regtest => "Regtest".to_string(),
        }
    }
}

impl From<NetworkWASM> for Network {
    fn from(network: NetworkWASM) -> Self {
        match network {
            NetworkWASM::Mainnet => Network::Dash,
            NetworkWASM::Testnet => Network::Testnet,
            NetworkWASM::Devnet => Network::Devnet,
            NetworkWASM::Regtest => Network::Regtest,
        }
    }
}

impl TryFrom<u8> for NetworkWASM {
    type Error = napi::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NetworkWASM::Mainnet),
            1 => Ok(NetworkWASM::Testnet),
            2 => Ok(NetworkWASM::Devnet),
            3 => Ok(NetworkWASM::Regtest),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid network value",
            )),
        }
    }
}

impl TryFrom<String> for NetworkWASM {
    type Error = napi::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "mainnet" => Ok(NetworkWASM::Mainnet),
            "testnet" => Ok(NetworkWASM::Testnet),
            "devnet" => Ok(NetworkWASM::Devnet),
            "regtest" => Ok(NetworkWASM::Regtest),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid network value",
            )),
        }
    }
}

impl TryFrom<DynamicValue> for NetworkWASM {
    type Error = napi::Error;

    fn try_from(value: DynamicValue) -> Result<Self, Self::Error> {
        match value {
            DynamicValue::Text(str) => NetworkWASM::try_from(str),
            DynamicValue::Uint8(num) => NetworkWASM::try_from(num),
            DynamicValue::Uint16(num) => NetworkWASM::try_from(num as u8),
            DynamicValue::Uint32(num) => NetworkWASM::try_from(num as u8),
            DynamicValue::Uint64(num_str) => NetworkWASM::try_from(num_str.try_to_u64()? as u8),
            _ => Err(napi::Error::new(
                Status::InvalidArg,
                "Invalid platform version value",
            )),
        }
    }
}

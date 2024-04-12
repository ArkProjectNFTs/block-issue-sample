pub mod byte_array;
pub mod cairo_string_parser;
pub mod client;

use anyhow::Result;
use num_bigint::BigUint;
use num_traits::Num;
use starknet::core::types::EmittedEvent;
use std::collections::HashMap;
use std::fmt::LowerHex;

pub fn to_hex_str<T: LowerHex>(value: &T) -> String {
    format!("0x{:064x}", value)
}

#[derive(Debug, Clone)]
pub struct CairoU256 {
    pub low: u128,
    pub high: u128,
}

#[derive(Debug, Clone)]
pub struct EventResult {
    pub events: HashMap<u64, Vec<EmittedEvent>>,
    pub continuation_token: Option<String>,
}

impl CairoU256 {
    pub fn to_biguint(&self) -> BigUint {
        let low_bytes = self.low.to_be_bytes();
        let high_bytes = self.high.to_be_bytes();

        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(high_bytes);
        bytes.extend(low_bytes);

        BigUint::from_bytes_be(&bytes[..])
    }

    pub fn to_hex(&self) -> String {
        let token_id_big_uint = self.to_biguint();
        to_hex_str(&token_id_big_uint)
    }

    pub fn to_decimal(&self, padded: bool) -> String {
        let token_id_big_uint = self.to_biguint();
        let token_id_str: String = token_id_big_uint.to_str_radix(10);

        if padded {
            format!("{:0>width$}", token_id_str, width = 78)
        } else {
            token_id_str
        }
    }

    pub fn from_hex_be(value: &str) -> Result<Self> {
        // Remove the "0x" prefix if it exists
        let value = value.strip_prefix("0x").unwrap_or(value);

        // Parse the hexadecimal string into a BigUint
        let biguint = match BigUint::from_str_radix(value, 16) {
            Ok(b) => b,
            Err(_) => return Err(anyhow::anyhow!("Invalid hexadecimal string")),
        };

        // Convert the BigUint to a 32-byte buffer
        let mut bytes = biguint.to_bytes_be();
        let padding = vec![0; 32 - bytes.len()];
        bytes.splice(0..0, padding);

        // Split the input array into two parts
        let (high, low) = bytes.split_at(16);

        let low = u128::from_be_bytes(low.try_into()?);
        let high = u128::from_be_bytes(high.try_into()?);

        Ok(Self { low, high })
    }
}

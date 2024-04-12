use anyhow::Result;
use starknet::core::{types::FieldElement, utils::parse_cairo_short_string};
use std::ops::Add;

use super::byte_array::ByteArray;

#[derive(Debug)]
pub enum ParseError {
    NoValueFound,
    ShortStringError,
    ByteArrayError,
}

/// Parse a Cairo "long string" represented as a Vec of FieldElements into a Rust String.
///
/// # Arguments
/// * `field_elements`: A vector of FieldElements representing the Cairo long string.
///
/// # Returns
/// * A `Result` which is either the parsed Rust string or an error.
pub fn parse_cairo_string(field_elements: Vec<FieldElement>) -> Result<String, ParseError> {
    match field_elements.len() {
        0 => Err(ParseError::NoValueFound),
        // If the long_string contains only one FieldElement, try to parse it using the short string parser.
        1 => match field_elements.first() {
            Some(first_string_field_element) => {
                match parse_cairo_short_string(first_string_field_element) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(ParseError::ShortStringError),
                }
            }
            None => Err(ParseError::NoValueFound),
        },
        // If the long_string has more than one FieldElement, parse each FieldElement sequentially
        // and concatenate their results.
        len => {
            let first_element = field_elements.first().unwrap();

            let a_size = first_element
                .add(FieldElement::ONE)
                .to_string()
                .parse::<usize>()
                .unwrap();

            if len == a_size {
                let results: Result<Vec<_>, _> = field_elements[1..]
                    .iter()
                    .map(parse_cairo_short_string)
                    .collect();

                results
                    .map(|strings| strings.concat())
                    .map_err(|_| ParseError::ShortStringError)
            } else {
                let data = field_elements[1..field_elements.len() - 2].to_vec();
                let pending_word = field_elements[field_elements.len() - 2];
                let pending_word_len = field_elements[field_elements.len() - 1];
                let pending_word_len = pending_word_len.to_string().parse::<usize>().unwrap();

                let byte_array = ByteArray {
                    data,
                    pending_word,
                    pending_word_len,
                };

                byte_array
                    .to_string()
                    .map_err(|_| ParseError::ByteArrayError)
            }
        }
    }
}

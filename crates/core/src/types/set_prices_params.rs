use serde::Serialize;
use starknet::core::types::{EthAddress, FieldElement};

use crate::types::field_element_utils::IntoFieldElementVec;

#[derive(Debug, Serialize)]
pub struct SetPricesParams {
    pub signer_info: u128,
    pub tokens: Vec<EthAddress>,
    pub compacted_min_oracle_block_numbers: Vec<u64>,
    pub compacted_max_oracle_block_numbers: Vec<u64>,
    pub compacted_oracle_timestamps: Vec<u64>,
    pub compacted_decimals: Vec<u128>,
    pub compacted_min_prices: Vec<u128>,
    pub compacted_min_prices_indexes: Vec<u128>,
    pub compacted_max_prices: Vec<u128>,
    pub compacted_max_prices_indexes: Vec<u128>,
    pub signatures: Vec<FieldElement>,
    pub price_feed_tokens: Vec<EthAddress>,
}

impl From<&SetPricesParams> for Vec<FieldElement> {
    fn from(item: &SetPricesParams) -> Self {
        let mut field_elements = Vec::new();

        field_elements.push(FieldElement::from(item.signer_info));
        field_elements.extend(item.tokens.as_field_element_vec());
        field_elements.extend(
            item.compacted_min_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend(
            item.compacted_max_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend(item.compacted_oracle_timestamps.as_field_element_vec());
        field_elements.extend(item.compacted_decimals.as_field_element_vec());
        field_elements.extend(item.compacted_min_prices.as_field_element_vec());
        field_elements.extend(item.compacted_min_prices_indexes.as_field_element_vec());
        field_elements.extend(item.compacted_max_prices.as_field_element_vec());
        field_elements.extend(item.compacted_max_prices_indexes.as_field_element_vec());
        field_elements.extend(item.signatures.clone());
        field_elements.extend(item.price_feed_tokens.as_field_element_vec());

        field_elements
    }
}

impl From<SetPricesParams> for Vec<FieldElement> {
    fn from(item: SetPricesParams) -> Self {
        (&item).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::{EthAddress, FieldElement};

    #[test]
    fn test_set_prices_params_to_vec_field_elements() {
        let tokens =
            vec![EthAddress::from_hex("0x41C7DD48b7D4efBfCD258F09574B297027Cae305").unwrap()];
        let compacted_min_oracle_block_numbers = vec![1, 2, 3];
        let compacted_max_oracle_block_numbers = vec![4, 5, 6];
        let compacted_oracle_timestamps = vec![7, 8, 9];
        let compacted_decimals = vec![10, 11, 12];
        let compacted_min_prices = vec![13, 14, 15];
        let compacted_min_prices_indexes = vec![16, 17, 18];
        let compacted_max_prices = vec![19, 20, 21];
        let compacted_max_prices_indexes = vec![22, 23, 24];
        let signatures = vec![FieldElement::from(25_u8), FieldElement::from(26_u8)];
        let price_feed_tokens =
            vec![EthAddress::from_hex("0x41C7DD48b7D4efBfCD258F09574B297027Cae305").unwrap()];

        let set_prices_params = SetPricesParams {
            signer_info: 1,
            tokens,
            compacted_min_oracle_block_numbers,
            compacted_max_oracle_block_numbers,
            compacted_oracle_timestamps,
            compacted_decimals,
            compacted_min_prices,
            compacted_min_prices_indexes,
            compacted_max_prices,
            compacted_max_prices_indexes,
            signatures,
            price_feed_tokens,
        };

        let expected = vec![
            FieldElement::from(1_u8),
            FieldElement::from(
                EthAddress::from_hex("0x41C7DD48b7D4efBfCD258F09574B297027Cae305").unwrap(),
            ),
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(3_u8),
            FieldElement::from(4_u8),
            FieldElement::from(5_u8),
            FieldElement::from(6_u8),
            FieldElement::from(7_u8),
            FieldElement::from(8_u8),
            FieldElement::from(9_u8),
            FieldElement::from(10_u8),
            FieldElement::from(11_u8),
            FieldElement::from(12_u8),
            FieldElement::from(13_u8),
            FieldElement::from(14_u8),
            FieldElement::from(15_u8),
            FieldElement::from(16_u8),
            FieldElement::from(17_u8),
            FieldElement::from(18_u8),
            FieldElement::from(19_u8),
            FieldElement::from(20_u8),
            FieldElement::from(21_u8),
            FieldElement::from(22_u8),
            FieldElement::from(23_u8),
            FieldElement::from(24_u8),
            FieldElement::from(25_u8),
            FieldElement::from(26_u8),
            FieldElement::from(
                EthAddress::from_hex("0x41C7DD48b7D4efBfCD258F09574B297027Cae305").unwrap(),
            ),
        ];
        let out: Vec<FieldElement> = set_prices_params.into();

        assert_eq!(out, expected);
    }
}

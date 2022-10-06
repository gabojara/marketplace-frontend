use marketplace_domain::{ContributorAccountAddress, HexPrefixedString};
use starknet::core::types::FieldElement;

pub trait TryFromContributorAccount: Sized {
	type Error;
	fn try_from_contributor_account_address(
		value: ContributorAccountAddress,
	) -> Result<Self, Self::Error>;
}

impl TryFromContributorAccount for FieldElement {
	type Error = anyhow::Error;

	fn try_from_contributor_account_address(
		contributor_account_address: ContributorAccountAddress,
	) -> Result<Self, Self::Error> {
		let hex_string: HexPrefixedString = contributor_account_address.into();
		FieldElement::from_hex_be(&hex_string.to_string()).map_err(anyhow::Error::msg)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use marketplace_domain::HexPrefixedString;
	use rstest::rstest;
	use std::str::FromStr;

	#[rstest]
	fn try_from_contributor_account() {
		let string: ContributorAccountAddress =
			HexPrefixedString::from_str("0x112233").unwrap().into();
		assert_eq!(string.to_string(), "0x00112233");
		let felt: FieldElement =
			FieldElement::try_from_contributor_account_address(string).unwrap();
		assert_eq!(felt.to_string(), "1122867");
	}
}

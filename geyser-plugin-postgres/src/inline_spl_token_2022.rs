/// Partial Solarti Token declarations inlined to avoid an external dependency on the solarti-token-2022 crate
/// Copied from miraland-runtime
use crate::inline_spl_token::{self, GenericTokenAccount};

solana_sdk::declare_id!("Token8N5ecJeFxL83iFa2h7AgJ8AtufM7bbg63LrW89");

// `spl_token_program_2022::extension::AccountType::Account` ordinal value
const ACCOUNTTYPE_ACCOUNT: u8 = 2;

pub struct Account;
impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        inline_spl_token::Account::valid_account_data(account_data)
            || ACCOUNTTYPE_ACCOUNT
                == *account_data
                    .get(inline_spl_token::Account::get_packed_len())
                    .unwrap_or(&0)
    }
}

/// Partial Solarti Token declarations inlined to avoid an external dependency on the solarti-token crate
/// Copied from miraland-runtime
use solana_sdk::pubkey::{Pubkey, PUBKEY_BYTES};

solana_sdk::declare_id!("Token4Q2B47VCdUy8u3rSTMMk2bGA1k7eN8qfKSzdiM");

pub(crate) mod new_token_program {
    solana_sdk::declare_id!("gntoZ1nNAAvXR1jcoXWi1SKLdN8gXR3pg6P9YdUXmnE");
}

/*
    /// The Solarti token definition -- we care about only the mint and owner fields for now.
    /// at offset 0 and 32 respectively.
    spl_token::state::Account {
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: COption<Pubkey>,
        state: AccountState,
        is_native: COption<u64>,
        delegated_amount: u64,
        close_authority: COption<Pubkey>,
    }
*/
pub const SPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SPL_TOKEN_ACCOUNT_OWNER_OFFSET: usize = 32;
const SPL_TOKEN_ACCOUNT_LENGTH: usize = 165;

pub(crate) trait GenericTokenAccount {
    fn valid_account_data(account_data: &[u8]) -> bool;

    // Call after account length has already been verified
    fn unpack_account_owner_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, SPL_TOKEN_ACCOUNT_OWNER_OFFSET)
    }

    // Call after account length has already been verified
    fn unpack_account_mint_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, SPL_TOKEN_ACCOUNT_MINT_OFFSET)
    }

    // Call after account length has already been verified
    fn unpack_pubkey_unchecked(account_data: &[u8], offset: usize) -> &Pubkey {
        // bytemuck::from_bytes(&account_data[offset..offset + PUBKEY_BYTES])
        // MI: to avoid error: integer arithmetic detected in CI due to clippy
        bytemuck::from_bytes(&account_data[offset..offset.checked_add(PUBKEY_BYTES).unwrap()])
    }

    fn unpack_account_owner(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_account_data(account_data) {
            Some(Self::unpack_account_owner_unchecked(account_data))
        } else {
            None
        }
    }

    fn unpack_account_mint(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_account_data(account_data) {
            Some(Self::unpack_account_mint_unchecked(account_data))
        } else {
            None
        }
    }
}

pub struct Account;
impl Account {
    pub fn get_packed_len() -> usize {
        SPL_TOKEN_ACCOUNT_LENGTH
    }
}

impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        account_data.len() == SPL_TOKEN_ACCOUNT_LENGTH
    }
}

pub mod native_mint {
    solana_sdk::declare_id!("MLN1111111111111111111111111111111111111111");

    /*
        Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
    */
    pub const ACCOUNT_DATA: [u8; 82] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

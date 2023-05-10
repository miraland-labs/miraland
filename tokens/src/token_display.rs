use {
    miraland_account_decoder::parse_token::real_number_string_trimmed,
    solana_sdk::native_token::lamports_to_mln,
    std::{
        fmt::{Debug, Display, Formatter, Result},
        ops::Add,
    },
};

// MI: change symbol to 𝇊 for MLN
const MLN_SYMBOL: &str = "𝇊";

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Mln,
    SplToken,
}

pub struct Token {
    amount: u64,
    decimals: u8,
    token_type: TokenType,
}

impl Token {
    fn write_with_symbol(&self, f: &mut Formatter) -> Result {
        match &self.token_type {
            TokenType::Mln => {
                let amount = lamports_to_mln(self.amount);
                write!(f, "{}{}", MLN_SYMBOL, amount)
            }
            TokenType::SplToken => {
                let amount = real_number_string_trimmed(self.amount, self.decimals);
                write!(f, "{} tokens", amount)
            }
        }
    }

    pub fn mln(amount: u64) -> Self {
        Self {
            amount,
            decimals: 9,
            token_type: TokenType::Mln,
        }
    }

    pub fn spl_token(amount: u64, decimals: u8) -> Self {
        Self {
            amount,
            decimals,
            token_type: TokenType::SplToken,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Self) -> Self {
        if self.token_type == other.token_type {
            Self {
                amount: self.amount + other.amount,
                decimals: self.decimals,
                token_type: self.token_type,
            }
        } else {
            self
        }
    }
}

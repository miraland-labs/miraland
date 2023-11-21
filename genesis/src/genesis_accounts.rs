use {
    crate::{
        stakes::{create_and_add_stakes, StakerInfo},
        unlocks::UnlockInfo,
    },
    solana_sdk::{genesis_config::GenesisConfig, native_token::LAMPORTS_PER_MLN},
};

// MI: re-worked
// 9 month schedule is 100% after 9 months
const UNLOCKS_ALL_AT_9_MONTHS: UnlockInfo = UnlockInfo {
    cliff_fraction: 1.0,
    cliff_years: 0.75,
    unlocks: 0,
    unlock_years: 0.0,
    custodian: "F11m2rEkVqpcSMF2ApsQ7LCMWTLG6Xtm1GE5WzfZrSVQ",
};

// 9 month schedule is 50% after 9 months, then monthly for 2 years
const UNLOCKS_HALF_AT_9_MONTHS: UnlockInfo = UnlockInfo {
    cliff_fraction: 0.5,
    cliff_years: 0.75,
    unlocks: 24,
    unlock_years: 1.0 / 12.0,
    custodian: "F11m2rEkVqpcSMF2ApsQ7LCMWTLG6Xtm1GE5WzfZrSVQ",
};

// no lockups
const UNLOCKS_ALL_DAY_ZERO: UnlockInfo = UnlockInfo {
    cliff_fraction: 1.0,
    cliff_years: 0.0,
    unlocks: 0,
    unlock_years: 0.0,
    custodian: "F11m2rEkVqpcSMF2ApsQ7LCMWTLG6Xtm1GE5WzfZrSVQ",
};

pub const STABILIZER_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "deepsea needle",
        staker: "5MqqBwVWMyTLFyAxxSBVDiDNZH1wHYzj5yQefRgrRapc",
        lamports: 25_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("2LCER8y5Y7Nt34Ps4FsQeFSCnJqg36EbvGnztQfhBNNa"),
    },
    StakerInfo {
        name: "beverly hills",
        staker: "5W8XjsNA54n53qypEZQFC1PAuKxbtx5txGTm45DaSigR",
        lamports: 25_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("H8vknfsXVWhpHmsCfQxe6sqsJs9C3RGjjKD7BeTc9yvn"),
    },
    StakerInfo {
        name: "princeton ai lurker",
        staker: "BLWSzTjvvECM9RKBtMAN56kXDyTuQHid9jkjEKsiNTzZ",
        lamports: 25_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("4wAmYXa9LGx7DCQcu2NkvxH5aiiaAKJDjPuVfnVgB3HC"),
    },
];

pub const CREATOR_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "spring equinox",
        staker: "Bv397MHXQoL9MmN9owRTkoqCUyND8vNtkGqumsHt6d9i",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("DNNyhS7Vm8LLWm6JAF5rH2XuJrSRHKPwDLfJnvPtmgnP"),
    },
    StakerInfo {
        name: "summer solstice",
        staker: "2Wiw47SCNWrXZHfY7mqyZ6HDzoTQF9MrxsoMzy4Euiyb",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("FTvTJeRnLLyqHtmaKZjMecDFUAdw57yb7UU63seNP8oV"),
    },
    StakerInfo {
        name: "autumn equinox",
        staker: "6zaPqDPKkXmmQhUc7MdVFQngKmvPJmvbG5hZ8tqvC2oZ",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("28TMZXuRt22phNL663qoxdQn3EPPJc9ZnxwpMbPW9Dj9"),
    },
    StakerInfo {
        name: "winter solstice",
        staker: "59BM7ZmLRHt9boiyxyWKZBPKdY5prFNgKSAW3MFAAfCv",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("FPzfyoKfT8mAQZ1NKxfWvwPhx5jaUtVSJ2i7pSfTLFtb"),
    },
    StakerInfo {
        name: "solarti ji",
        staker: "8Ts9eg8pVZd3m7ffM4C66visKZtedQ2CXt3J7ojnnHjA",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("59zbUaZyRSUYbmBU7vkd1qrBy5sZt4L7gcuqcf68YfKz"),
    },
    StakerInfo {
        name: "start of spring",
        staker: "J6wxQ5foRuHxbjBDNS9qVJkDk4Ac3d8ZTCR7mGRjom7W",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8QfLiRZFzf1DKzUFnxgDfBM5eG2hWx2tEvwf1SfwL4Wo"),
    },
    StakerInfo {
        name: "start of summer",
        staker: "791mn7jx6Gb26AYeqivrf13FmEyHSoQSfUG67VcdmnkF",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("A2tMuCdupkeDpAdfqdEYLcqzxH7w37i4pqdybq6Xn9Gs"),
    },
    StakerInfo {
        name: "start of autumn",
        staker: "4JwqjygXDwmarRsTrEherCiY4JSARRPS58sAfR2YvPnF",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("uKLorMHAXVvwssteDn8p5gYPvMrm7Fs63kouMNMGRFU"),
    },
    StakerInfo {
        name: "start of winter",
        staker: "DYet6eSYW5ouxnT266u5akfaqeaqhnjmxM5xXfJ5Q5hx",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("66T3tXeyQGYESAT46kibKVbT23otH9iaTz7EUgjXz7Pf"),
    },
    StakerInfo {
        name: "rain water",
        staker: "8Mf4iGvwa3cSPiw341KTXeriGnimkRqGCqKNhYKKSZ2d",
        lamports: 500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("EAmq7S7pcU3McGeaF4tq5oeznXWg9YKoCZnE3WqYsLeJ"),
    },
    StakerInfo {
        name: "awakening from hibernation",
        staker: "6Zx8GFYQ46saX5Gqp9TtSeUo95sz5LEHoDkQWMqL7kGi",
        lamports: 500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("63SDbWeqxbndapRaZBtChwapBfZuybSgAk9DS89dciti"),
    },
    StakerInfo {
        name: "fresh green",
        staker: "5hvujsAtXFDZ7mVskLsNpChjMavGFPR29nj76d3yLYjH",
        lamports: 500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("4RwsjDuE7PSYxiep6tnHP7uVAWstWPxvoEM81UWkJetq"),
    },
    StakerInfo {
        name: "grain rain",
        staker: "6krftJNg1AhWAiuNQM8pLZY7EXuDCoozVs4DHZWajhgs",
        lamports: 500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("Eo6uYmaMYKWEM4qASNwBAwMJdx5tJx22PynVDKuZN7mZ"),
    },
    StakerInfo {
        name: "grain buds",
        staker: "FTse1oRz5V27QgJtGuhix4yAggntoUu4WuUcHDrYcHkx",
        lamports: 500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("Ced1ufbhSenPxAjRmRa4PhGC4NL61YV7kJQ27jGQLR3R"),
    },
    StakerInfo {
        name: "humpback whale breaching",
        staker: "S6c1D5a3GA5FxVPrWzUNEhEL5JQQrhMA53XCu6uP1Q9",
        lamports: 2_500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("DjHKVPBfpqKUThB4iSE2HGJp1bWcKdu7j2B8xkeDkNVt"),
    },
];

pub const SERVICE_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "pool of pioneers 1 to 20",
        staker: "A8BDtUskDTuaHfpdLpr8a3RM7Nc9U5j6BpSEdzfzqWVo",
        lamports: 2_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("BkZK1JQRYRQJQ9DkMGgdUfEoRWvREntYK57zcFfPA5qN"),
    },
    StakerInfo {
        name: "pool of pioneers 21 to 40",
        staker: "FpBHMk7bYpjSDx1Fmrmtsy6J12cKn5ooYZ1A9cnwZmDu",
        lamports: 1_400_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("5Lhdhq18qUFVHsLpKtbMDvde7E8ufsrqajAGKoB5i9D5"),
    },
    StakerInfo {
        name: "pool of pioneers 41 to 60",
        staker: "9772L9fedG8gygzZKRKGLzj64zveX5DQa2cttS4JCSDo",
        lamports: 800_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("5tTemvmLiC1MpN3tFEyF6XJ5C5c2r7jknfUbH1H8goUp"),
    },
    StakerInfo {
        name: "pool of pioneers 61 to 80",
        staker: "AKE4zCX6Gy5N4uQ7ztQGn5mSXMqmjXBzvbZNFtgUFN4m",
        lamports: 400_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("QYiWJYViSY7YwV4YMHceLoSDCUMdmAGA5BgVBZakZKg"),
    },
    StakerInfo {
        name: "pool of pioneers 81 to 100",
        staker: "4tS4nKocxPwWShh7KeV61aVMCDHyeVKG4Z3W11n7a8pd",
        lamports: 200_000 * LAMPORTS_PER_MLN,
        withdrawer: None, // MI: intended no separated withdrawer
    },
    StakerInfo {
        name: "pool of pioneers 101 to 200",
        staker: "8i6T8finS7SMGmHr6nMscBA36FivqVY7Mh13VGH1Qysj",
        lamports: 200_000 * LAMPORTS_PER_MLN,
        withdrawer: None, // MI: intended no separated withdrawer
    },
];

pub const FOUNDATION_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "lesser heat",
        staker: "6uJCwcAB8ZQDnfzYZGu98b46obhF6vD2gau4q1HmPHVs",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("Afx9JDp1kN4ECiTHVU8i25PXTvUGTBxwy91JGD9w7ryu"),
    },
    StakerInfo {
        name: "greater heat",
        staker: "64zyH1T57pN4ttU5UdVzibrd2zWmBmhPVVZViK1avn7B",
        lamports: 45_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8hw8TFzAJhHSxMwBxaPhvXiPH6hcqtbj7b2WhxpveKKW"),
    },
];

pub const GRANTS_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "white dew",
        staker: "HywWHFz4omZWKZoHurAbbpPg4eQ8b6XZ1Jnvj5pvmJta",
        lamports: 12_500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("3iHj6hsNJJUvTwpSfzb1L6qbN5rHDzHfWyrKtFm4AxCd"),
    },
    StakerInfo {
        name: "cold dew",
        staker: "7PMAkvauc2JsYyB2x4k1yCV8nkGeMM8dWnuGKYZmkpch",
        lamports: 12_500_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("32YYzaHgvnmgxCkvh5dfoTh2TZvbMxiAcfdT8ZnHbjDw"),
    },
];

pub const COMMUNITY_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "first frost",
        staker: "CRe7Kmxu1s8HcNbGssbRkY1An41PkE9BY2ZFTXLyCZDs",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("4aV7ZiKFr793G3ptxh1SLRti7YcPkokLkPgToNnBv8y1"),
    },
    StakerInfo {
        name: "light snow",
        staker: "D9DqHTEbpnpndG6Ze73VY8v3ELw6kRHEZvwBhWAy6ZXs",
        lamports: 35_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("GJYdbtB6As33MD6ETRnS1r4f7iP2Y4GDWk7WwDbPW96p"),
    },
    StakerInfo {
        name: "heavy snow",
        staker: "HUtz6e1N9EwX1LguQHGyndoa3ReJesqqvabsX7kiEgqk",
        lamports: 150_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8oUBRzRttx3LGjd9Ywa5ZDg9bbzvkcCMB9gc33KtZ1eX"),
    },
];

fn add_stakes(
    genesis_config: &mut GenesisConfig,
    staker_infos: &[StakerInfo],
    unlock_info: &UnlockInfo,
) -> u64 {
    staker_infos
        .iter()
        .map(|staker_info| create_and_add_stakes(genesis_config, staker_info, unlock_info, None))
        .sum::<u64>()
}

pub fn add_genesis_accounts(genesis_config: &mut GenesisConfig, mut issued_lamports: u64) {
    // add_stakes() and add_validators() award tokens for rent exemption and
    //  to cover an initial transfer-free period of the network

    issued_lamports += add_stakes(
        // extra category for stabilizer, m17
        genesis_config,
        STABILIZER_STAKER_INFOS,
        &UNLOCKS_HALF_AT_9_MONTHS,
    ) + add_stakes(
        genesis_config,
        CREATOR_STAKER_INFOS,
        &UNLOCKS_HALF_AT_9_MONTHS,
    ) + add_stakes(
        genesis_config,
        SERVICE_STAKER_INFOS,
        &UNLOCKS_ALL_AT_9_MONTHS,
    ) + add_stakes(
        genesis_config,
        FOUNDATION_STAKER_INFOS,
        &UNLOCKS_ALL_DAY_ZERO,
    ) + add_stakes(genesis_config, GRANTS_STAKER_INFOS, &UNLOCKS_ALL_DAY_ZERO)
        + add_stakes(
            genesis_config,
            COMMUNITY_STAKER_INFOS,
            &UNLOCKS_ALL_DAY_ZERO,
        );

    // "one thanks" (community pool) gets 500_000_000SOL (total) - above distributions
    // MI: investors, top-ups for bootstrapper validators and other built-in accounts, etc.
    create_and_add_stakes(
        genesis_config,
        &StakerInfo {
            name: "one thanks",
            staker: "E1U44nhAfuTiDcKSFgAYLQF8eCFW5dM3jyc7ckQtW7D9",
            lamports: (500_000_000 * LAMPORTS_PER_MLN).saturating_sub(issued_lamports),
            withdrawer: Some("9nSWbgvEjPXw1hiqpsjbzxsD4sLh8qnnASxjBN3aU9UP"),
        },
        &UNLOCKS_ALL_DAY_ZERO,
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_genesis_accounts() {
        let mut genesis_config = GenesisConfig::default();

        add_genesis_accounts(&mut genesis_config, 0);

        let lamports = genesis_config
            .accounts
            .values()
            .map(|account| account.lamports)
            .sum::<u64>();

        assert_eq!(500_000_000 * LAMPORTS_PER_MLN, lamports);
    }
}

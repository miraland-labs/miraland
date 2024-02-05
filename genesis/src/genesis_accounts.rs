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
    custodian: "FKSMis6Q8JfDUS1hMKZSPLoRSdpT3EZyF1VTJFTbgFQ9",
};

// 9 month schedule is 50% after 9 months, then monthly for 2 years
const UNLOCKS_HALF_AT_9_MONTHS: UnlockInfo = UnlockInfo {
    cliff_fraction: 0.5,
    cliff_years: 0.75,
    unlocks: 24,
    unlock_years: 1.0 / 12.0,
    custodian: "FKSMis6Q8JfDUS1hMKZSPLoRSdpT3EZyF1VTJFTbgFQ9",
};

// no lockups
const UNLOCKS_ALL_DAY_ZERO: UnlockInfo = UnlockInfo {
    cliff_fraction: 1.0,
    cliff_years: 0.0,
    unlocks: 0,
    unlock_years: 0.0,
    custodian: "FKSMis6Q8JfDUS1hMKZSPLoRSdpT3EZyF1VTJFTbgFQ9",
};

pub const STABILIZER_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "deepsea needle",
        staker: "92qFxHTQW42snt4jUBhSSWL5ENJrVxTgWKiZXXnYAg82",
        lamports: 50_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("G15dPhKSoHoqtH4AuGMxwMMskJayKekNMmU9KAo7wqrf"),
    },
    StakerInfo {
        name: "beverly hills",
        staker: "HQzg7ysWa331dZxZ3zzT98nKC5s6WP1r9aC7CKsxVTdY",
        lamports: 30_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("9QYsafpifKUKCoiJTs14565Q4vEPZVrcL2BBz5bCXdQr"),
    },
    StakerInfo {
        name: "princeton ai lurker",
        staker: "H4ZqwqiVPChxLx4LHbth1gENNUYwzQuxi4Rf1rPkghXR",
        lamports: 20_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("3NUQeCuE68hVBTWzKqiojTyqzS8ADBU8JV65nJuVzLnK"),
    },
];

pub const CREATOR_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "spring equinox",
        staker: "x6fYyooyex9Lk9z2Ws6wLGvSTyjzggVx1ia6TrEA42F",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("9nTy1AAboLQrWbXCs96VEshFiGRZaj5DN6uNB5nbwxWn"),
    },
    StakerInfo {
        name: "summer solstice",
        staker: "HjVDPbavSuip8S55vzLntB6vh9cSaCi1Kzgrk1hcdjo7",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("Dpwzbdgyv7uLif23azeoF5ieoWbZse1ZExoDtEMsFfBe"),
    },
    StakerInfo {
        name: "autumn equinox",
        staker: "4DMkwtQ3xLN3vvNqNVBJYKyGAoXL49VdHtWiM4xnA7qb",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("CnnvdQ8Q3QBdaYo6gzqcQper3S7yb4rMWGcaaoBKx3Fr"),
    },
    StakerInfo {
        name: "winter solstice",
        staker: "GHQY8x3mqQPdohcpsahwVjfj5AfSscP6HAc3j7zsi5RV",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("J4DJ64KAqNM3ggnuo69q4QgMGrqjMn16iNGoUyuZwQRw"),
    },
    StakerInfo {
        name: "solarti ji",
        staker: "3NnF2ix1yKBqvDJukpT7R6SDbwhJ3B7nUjJsVFDeGGxv",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8JGJDQr5HJtNRLJPe5Fn5ANWqNzvaahou6ASUuyqA6dV"),
    },
    StakerInfo {
        name: "start of spring",
        staker: "Ev8AtVDcga1bui7pRqGuywMLDMNWEcSvfc7npMiRUKvG",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("9MzqJhmktF3LK1CT1zLU1xCSShW43HyU9BjhMAfD3ZTA"),
    },
    StakerInfo {
        name: "start of summer",
        staker: "3yuvvZu9yw7UNj48DPJwMmJpxR5q3MDN5an4yswJtzBy",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("2SV6iH5QZYTkZRwFy7sje3CS619tGBfE5xSqe8HWoEES"),
    },
    StakerInfo {
        name: "start of autumn",
        staker: "GMmsG13pLhSiq5bnLwLbxq3uvxKQsNUjzLQ2Mc4PKpZL",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("7MeEw8Pq5vBn6j6EEytiCmJh2CTNue6RidAEa88rXYtY"),
    },
    StakerInfo {
        name: "start of winter",
        staker: "2ptsfKrVUWipNgBJnymNo2nVL7f7oDBbxiTi7feYoCEk",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8wsC3G2mK6Ap6kyrmYPA3NGm4f8LNR5ap1fNH1iu2Mse"),
    },
    StakerInfo {
        name: "rain water",
        staker: "AoePuUBf3dDNvwtveVRaxaJmhLvL6CzE936foZD7JQ9M",
        lamports: 1_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("HxjQtd1R3AFNCqcWhGmLYYxrCcfSw9cQnwS887G25Z1p"),
    },
    StakerInfo {
        name: "awakening from hibernation",
        staker: "EbQebmor1nCT1qUNDmwJyvx59FvU1nNHdiV3VH7U6q7F",
        lamports: 1_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("8nuKfb2G2T8eqNGE9cLtQsxN7Mvstwe1YHtARTLkafeR"),
    },
    StakerInfo {
        name: "fresh green",
        staker: "61oAG2wPKsVgYY9a76xTZpgR13wEwc8Z9JN3pjctGmYw",
        lamports: 1_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("2s6dy3jPGdcThiE7rE1A8qKYw6G3fNGJL8XraFqFx3v2"),
    },
    StakerInfo {
        name: "grain rain",
        staker: "ByoHtMZLgvZpg52ZVmi9rwfkaT6DQQAtxoMb1PCoE8dX",
        lamports: 1_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("7AjkYaQRkkbMzPSNmZ7rQPeBRBJhSr85Gqe6UAxs2LUY"),
    },
    StakerInfo {
        name: "grain buds",
        staker: "8hfGnKTqkoB6eY4zfvUQjynS5WK8KdqjMA2RxUs3Gyrd",
        lamports: 1_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("3hUSrhaJNZZXUiNFsYx8c2QoA8aFaAXetFyjYMr4AMvN"),
    },
    StakerInfo {
        name: "humpback whale breaching",
        staker: "DjGjxB3xU6Hx8MA5vDB8vuuu8X7GepmDv8PDLmASsVwh",
        lamports: 5_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("J2MBdb9oxgp7KczKwomJ7L2XsKKPAMMWm9sXA7yeZsZf"),
    },
];

pub const SERVICE_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "pool of pioneers 1 to 20",
        staker: "BwnVuimfquQ9vwimEnp3PZkeaZ7xLbS9vYDoi41wmRx8",
        lamports: 4_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("6Dxv4JJXpPD5eB917E6HGDrjA8VgvtJU8kUMFvRBckae"),
    },
    StakerInfo {
        name: "pool of pioneers 21 to 40",
        staker: "3HiQZbF8nJk2uPbkTkyKBP7cJrZWZPUcrLsmp7yNbnWr",
        lamports: 2_800_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("ZU2bAL1TfEzWd4fq1FMhMxPjmDUEiXXhCueK5yPg5Te"),
    },
    StakerInfo {
        name: "pool of pioneers 41 to 60",
        staker: "3CbtAqZBq1gQc71hoAvHe4fXHEnccmajwvZuwUUkrkVY",
        lamports: 1_600_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("GwzvjTrfgAAnSjf6TgZdUtyUrpNxVDEYPjpYHksTuAA3"),
    },
    StakerInfo {
        name: "pool of pioneers 61 to 80",
        staker: "58fEGtq1XkLhAF6KqqoeMbc5kXXPfWTbJD3PWX7Y18Ce",
        lamports: 800_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("J9Y29buWBjHHzJq3PvT2jffXeAmXWEp8ej3XBkdytvNp"),
    },
    StakerInfo {
        name: "pool of pioneers 81 to 100",
        staker: "AeEqoPrpJS2ER3sf8jF9HhwZa8dhLbuydkwdwyezcCjd",
        lamports: 400_000 * LAMPORTS_PER_MLN,
        withdrawer: None, // MI: intended no separated withdrawer
    },
    StakerInfo {
        name: "pool of pioneers 101 to 200",
        staker: "5WApNju6SfFkZmU8jWPwxGxoeQbMSAbGvJXLSbdK9twv",
        lamports: 400_000 * LAMPORTS_PER_MLN,
        withdrawer: None, // MI: intended no separated withdrawer
    },
];

pub const FOUNDATION_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "lesser heat",
        staker: "9ecn2hXQty6yA24zZBmB76D2GHiTwDt2Uk32xfrofn1v",
        lamports: 10_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("5WH3zKrNqoqp4SJsRkiD2LNhWpHdVuoLrBXQGZDYvy3o"),
    },
    StakerInfo {
        name: "greater heat",
        staker: "DsswPkujUTNhhLiABZ88dtarhjzreAiLFFJbucZZ7oha",
        lamports: 90_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("C9kWuwLfMBhgUhgo99NBFhB79bZ5YKy8BNdfaNT5Mjf5"),
    },
];

pub const GRANTS_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "white dew",
        staker: "3FCjcqPFCwUyqAxmcDfezWYSxkWCjcT1wFQrYPJddc9x",
        lamports: 25_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("DRZt9xVWwkUYYMPmX1YGfN7m18prYEEjbFw7kn8SFt1q"),
    },
    StakerInfo {
        name: "cold dew",
        staker: "zCoDm1GK3PBTM7BTmUA1ZmMGPMRDE8dpP3b48cRMR8X",
        lamports: 25_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("CCfRpyY6FciFciKxAYXNR69tf58bCrUXMNvHnj1zDyhr"),
    },
];

pub const COMMUNITY_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "first frost",
        staker: "GYVzPkx76uFaCLsFJYj12VEstKeHdc6fAFHzrQFnbjdB",
        lamports: 50_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("6w28EvVYqtn3D6eaTXCTBNAEAcuCmqNbThUU6XkvD54Q"),
    },
    StakerInfo {
        name: "light snow",
        staker: "HMBusRR1oPtnZUa8TVWdfBVtE45sYUdPX72sPseQ8Zaj",
        lamports: 100_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("HYriosG8ZpQqSD198HCfjMLehb2LAK4gEHQmVu2aWq43"),
    },
    StakerInfo {
        name: "heavy snow",
        staker: "FTHazyBqHhFwLrtQGnGGtd7APSrJcgtgm7z7F9WVJvBR",
        lamports: 300_000_000 * LAMPORTS_PER_MLN,
        withdrawer: Some("ER8rXyaD9pKy2gsrLyQ28zo8hUjVMPuSF6x82DRgW29d"),
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

    // "one thanks" (community pool) gets 500_000_000MLN (total) - above distributions
    // MI: investors, top-ups for bootstrapper validators and other built-in accounts, etc.
    create_and_add_stakes(
        genesis_config,
        &StakerInfo {
            name: "one thanks",
            staker: "ACB1hAV6FNmkqbhDYLzUPFPhwZjdssWuqLiPsgywHc4z",
            lamports: (1_000_000_000 * LAMPORTS_PER_MLN).saturating_sub(issued_lamports),
            withdrawer: Some("F7tnwUsKngLFdPdM6Thqomvzkw6ckox2An1huXnBBFFt"),
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

        assert_eq!(1_000_000_000 * LAMPORTS_PER_MLN, lamports);
    }
}

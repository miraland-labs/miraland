//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core Miraland code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://spl.solana.com/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `miraland-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contributor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `solana_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    solana_program::{epoch_schedule::EpochSchedule, stake_history::Epoch},
    solana_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod deprecate_rewards_sysvar {
    solana_sdk::declare_id!("2c1LfwhtrnYYUTADd7jNfw2v25C9wdD7TxaNNeBL26Uo");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("79Rx9u8bp61qVXfgU77ConTZusQrJAVCLHKX12fXQFLM");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        solana_sdk::declare_id!("4SLokfWfRfEdgc9LTQPeqVSTKp1yT9EGxkvunw2kyRYk");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                solana_sdk::declare_id!("6LAz8hDNfU2wZQNvU5A486aLb4uMqMP71VJuLeqa8VH1");
            }
            pub mod enable {
                solana_sdk::declare_id!("APxJq5bJDVHvXvxgnSzynG4fcrnjBftgiHqryr8HGzeZ");
            }
        }
    }
}

pub mod secp256k1_program_enabled {
    solana_sdk::declare_id!("7jUAsmg1aiq7KUFJmgTbonwB7o1UkcPwhaxyvtoK6aLv");
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("9UeP8WVfxerQVsvS7CKPocQb3SicYhzeJdfPxbbw3Xba");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("28yzxUrgESLBv8MLCq7HHWt69PqQm8qEFfdN9niqLx1n");
}

pub mod filter_stake_delegation_accounts {
    solana_sdk::declare_id!("2pjiimKVUKEgnc79asiV1SQLQ4jtHziCw8RzXtjHgfz5");
}

pub mod require_custodian_for_locked_stake_authorize {
    solana_sdk::declare_id!("BoSRYC39THTKeiGzUAfBJUJTUBpmvprqMdqbSaPmKf79");
}

pub mod spl_token_v2_self_transfer_fix {
    solana_sdk::declare_id!("BXTRt14DE2ogunj5E3KTU3Gp5aYJVNJAiswq8ocSbDnc");
}

pub mod warp_timestamp_again {
    solana_sdk::declare_id!("7jj4R75Y3zUvdfLSDcx4FXnWSPauxA3e5ajdxwGSq9HM");
}

pub mod check_init_vote_data {
    solana_sdk::declare_id!("8uSS6mrHnYcFF9xdLEuJBQ2zvePVtHmeoPnewXy6EzH1");
}

pub mod secp256k1_recover_syscall_enabled {
    solana_sdk::declare_id!("8bsFk18mhd2R81MkdXaWYNQGiDcDWDHcXxEnc3wmaPJ9");
}

pub mod system_transfer_zero_check {
    solana_sdk::declare_id!("F2VaF6pkmUnPUE92Pv6sephEWBkoUUi5evPUtNtxEDo9");
}

pub mod blake3_syscall_enabled {
    solana_sdk::declare_id!("4jNWNYf2uHsZ7DmP6RizeC9m5WiffkyGTtj6BehjBsbJ");
}

pub mod dedupe_config_program_signers {
    solana_sdk::declare_id!("7oQw7GQWdsUSbVqMqP42nTntnDkKJPPSM3ydrMSLCWAY");
}

pub mod verify_tx_signatures_len {
    solana_sdk::declare_id!("2V95BUaoS7mUgd4e6LCXtAd5WLuDyYFP5u9AuXfSZFHQ");
}

pub mod vote_stake_checked_instructions {
    solana_sdk::declare_id!("FBgh8jqxHJWJG9Rrc59gWb1bQq6PHFg675QVVitZtXMt");
}

pub mod rent_for_sysvars {
    solana_sdk::declare_id!("D2UrJpLDUzjSC7F7U3ZZpAw4UvZkSLybteeyL2Soa6AL");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    solana_sdk::declare_id!("2Tf8EAMRjo8Z9kEmSL5JLbf33Q2tjCSieP4FBZGrdHQZ");
}

pub mod tx_wide_compute_cap {
    solana_sdk::declare_id!("4AdEYJECdV8Zve71m7xLimhgdL3qi2tvqEy6r1VZm4ih");
}

pub mod spl_token_v2_set_authority_fix {
    solana_sdk::declare_id!("N2UX7hfPmEUQrmTKUqUqX3fspnsehrbAqyDzY5JFkcc");
}

pub mod merge_nonce_error_into_system_error {
    solana_sdk::declare_id!("5rgeSmE17s1tbUSDmmcqXL26A9SqaLNUmgDzKbDKY3af");
}

pub mod disable_fees_sysvar {
    solana_sdk::declare_id!("28GpzueRzsKyPd2vmkwhXKdr174viqEaMPoUaFBRXXRo");
}

pub mod stake_merge_with_unmatched_credits_observed {
    solana_sdk::declare_id!("5Y4VcaQSoUj4j9ZeLjq3fH2WRc8mbk3H5BH6Vf578wfF");
}

pub mod zk_token_sdk_enabled {
    solana_sdk::declare_id!("FNEsWEdUNGbyWVxiJrSmf1SbmypFJPtPURAGgcUiE7Xf");
}

pub mod curve25519_syscall_enabled {
    solana_sdk::declare_id!("8rd1cN2QwtAMR5BV4fEZGxyUfWJtSB2AihHhctPGc4Ya");
}

pub mod versioned_tx_message_enabled {
    solana_sdk::declare_id!("4R9FuBD4tN3rzYnGL6VLtehzuVXSbxgUSor4sRqYfxQD");
}

pub mod libsecp256k1_fail_on_bad_count {
    solana_sdk::declare_id!("ADf1p7q3ZHsmNuCn5SHh2AFdhrvP6UVBphsnURGy94eJ");
}

pub mod libsecp256k1_fail_on_bad_count2 {
    solana_sdk::declare_id!("4EBpJoi9YamhAY9hgPxFL3N2xk7GbiRWpfEcXgJZCzVx");
}

pub mod instructions_sysvar_owned_by_sysvar {
    solana_sdk::declare_id!("83x8QZSPCDqiCrfVhTPn4j6ML4iTSG6DgD1THk8rDnHJ");
}

pub mod stake_program_advance_activating_credits_observed {
    solana_sdk::declare_id!("ExSUs7m2qv6d4TYb72EuaKEBqcA8TktnRNm2GEeWWFP3");
}

pub mod credits_auto_rewind {
    solana_sdk::declare_id!("7iCvJnUS8kmC2gXARop5SV4TWgEu9VW3HCjPCX3aH6ap");
}

pub mod demote_program_write_locks {
    solana_sdk::declare_id!("EzXHe4N1ezMU1mAXkEbTxz6bT7NT8nZrD3yWaTdm5nKi");
}

pub mod ed25519_program_enabled {
    solana_sdk::declare_id!("8N6nBvvMTnC5FFNgBnFagpLo1ecNoCpvfkgFJVhVRHFY");
}

pub mod return_data_syscall_enabled {
    solana_sdk::declare_id!("6WTDFppf83DMNejYPQpFfniJSUocgNfyF6KDeriGy8be");
}

pub mod reduce_required_deploy_balance {
    solana_sdk::declare_id!("6V5vr414z1BXCJo9MXXArJNVKbkqtAH7HE32U6rcZESx");
}

pub mod sol_log_data_syscall_enabled {
    solana_sdk::declare_id!("3nsrptKRNefv1f7yHUAg9hbnf28GAkVGajaR42XEDV9Q");
}

pub mod stakes_remove_delegation_if_inactive {
    solana_sdk::declare_id!("9ryFTNZjt9eiGxPxZP9vmu1o7MTP432F4Deu3CBr8Fuf");
}

pub mod do_support_realloc {
    solana_sdk::declare_id!("DM4TyMJkUYN6YXVaeFpWTDGBdWo5sbhk82RDScK7ptjU");
}

// Note: when this feature is cleaned up, also remove the secp256k1 program from
// the list of builtins and remove its files from /programs
pub mod prevent_calling_precompiles_as_programs {
    solana_sdk::declare_id!("9EmqPEbM54LjSwHwNFBvF5G5ePmyc9vtYDdeafAXcUAB");
}

pub mod optimize_epoch_boundary_updates {
    solana_sdk::declare_id!("DkLJm8F8bXJJt4PjrXYRTrdvqJfrV9nKRWbLvKqBpTVV");
}

pub mod remove_native_loader {
    solana_sdk::declare_id!("7RtzXKHPtYjVLxG4Si7P28rB5eFtFA96ruZFCbSRMZjW");
}

pub mod send_to_tpu_vote_port {
    solana_sdk::declare_id!("EfZQW1z4UugTsjpL4q7kXEhM2cuokXAyx3xNimAX96hh");
}

pub mod requestable_heap_size {
    solana_sdk::declare_id!("2uWChP7zecD9mCWRN671u9RQBppxaJUvey1tbS5gEGJh");
}

pub mod disable_fee_calculator {
    solana_sdk::declare_id!("G7qQS6VDvdrGX2t9n8WGLKEXorCEFjdepyTWVc5FNPBp");
}

pub mod add_compute_budget_program {
    solana_sdk::declare_id!("35BUe4UVLer2dwHQyBn6EgRSwc2yfbvy9TQxD7vYRExU");
}

pub mod nonce_must_be_writable {
    solana_sdk::declare_id!("6z5xsisAAiauaE9cHiHrZAxajJyxFMkwTtaPsLGhCfNg");
}

pub mod spl_token_v3_3_0_release {
    solana_sdk::declare_id!("A3VfsyhcwZKkfTXN1a4Z1dSMQxWiaqjymPMBky2Lh5Mq");
}

pub mod leave_nonce_on_success {
    solana_sdk::declare_id!("CFtA7BGeyWgqh3y5EmgGsdHBCVqDrfAxC8Tm2mBuYox");
}

pub mod reject_empty_instruction_without_program {
    solana_sdk::declare_id!("CVnCWV9rZrdK9gawJ5DPXUggLufqzkiYpe7V8ewHv2gX");
}

pub mod fixed_memcpy_nonoverlapping_check {
    solana_sdk::declare_id!("3SxofBxWgtdYHEKnVEfcuCfmFqmFLiMeD7j4aE7NGJLg");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    solana_sdk::declare_id!("5at3F6XAWJRSdURRrhcPKMsWUu7jpErffQvhbudAG91p");
}

pub mod evict_invalid_stakes_cache_entries {
    solana_sdk::declare_id!("Dj5KYGDEE6XNZ2uuQxH1c9pDwhRS9X3kC7uTvXJnUtbC");
}

pub mod allow_votes_to_directly_update_vote_state {
    solana_sdk::declare_id!("9NQbXmRjHa4LnJVcmCYCQxTAEmgWASZB6dyR8ujmPift");
}

pub mod cap_accounts_data_len {
    solana_sdk::declare_id!("7qx6TcurUwSt6mRkwLjDrJDSxsMnNXsqcrCNzZeNq236");
}

pub mod max_tx_account_locks {
    solana_sdk::declare_id!("56EYkVzpWjvvrVjdmCXSTxFSSK3HUVFKavLyB3H9HvpK");
}

pub mod require_rent_exempt_accounts {
    solana_sdk::declare_id!("FRJstrV5EAhseibFSAXm5ZmPnHdBNAqaT1sDyy9jsoma");
}

pub mod filter_votes_outside_slot_hashes {
    solana_sdk::declare_id!("5izPfs14BkNSkjBBuuQxHbEZomzzF8tvXcbnfqX2Exwn");
}

pub mod update_syscall_base_costs {
    solana_sdk::declare_id!("HmoYBViLHHKw9tqxjGq7Vy8kXzTxzw7RmbBkvcnJfxR8");
}

pub mod stake_deactivate_delinquent_instruction {
    solana_sdk::declare_id!("5NU3DmTjfbtGZ7gsUWFE8TzxZ7PRF5EvbHCuGJhqUh4B");
}

pub mod stake_redelegate_instruction {
    solana_sdk::declare_id!("HD2vR1ZZzjqTXzUyWoUjL5ZAVBpe4wNo1ToR2ogEbbgc");
}

pub mod vote_withdraw_authority_may_change_authorized_voter {
    solana_sdk::declare_id!("97DV3qAzqqPpiqQuZjf3GvKcwgTZwsSLjWnP5naoYhkF");
}

pub mod spl_associated_token_account_v1_0_4 {
    solana_sdk::declare_id!("J3V6Rmm8dFmcZBadzGLLdU7Rrg2rNKWkfmYsTdkeBvmu");
}

pub mod reject_vote_account_close_unless_zero_credit_epoch {
    solana_sdk::declare_id!("9Xn27E7PdcE8rHwkB79MknuoydhVNkEbZFvQMVDchjwq");
}

pub mod add_get_processed_sibling_instruction_syscall {
    solana_sdk::declare_id!("1XCfEQcYXoakjx6ULrMeidyvSozjT3gu1ee38u8ofEu");
}

pub mod bank_transaction_count_fix {
    solana_sdk::declare_id!("9nFiWjLxyKFRTidASf7pgMFCYPHiXQ2oLmD4h6hpRWCA");
}

pub mod disable_bpf_deprecated_load_instructions {
    solana_sdk::declare_id!("CvqsVybTcwM9yi99UGALDtyQ3FxkmsiSzRGLTwwoiow9");
}

pub mod disable_bpf_unresolved_symbols_at_runtime {
    solana_sdk::declare_id!("FmksuagiZ2tXDWoa8o6Cjs3bTFU7vmqhyaYdRbuJdgwM");
}

pub mod record_instruction_in_transaction_context_push {
    solana_sdk::declare_id!("Eg3fSoHWFweKmwpbJnrdMmbebHZ5QpYrprcYtAhNpjmt");
}

pub mod syscall_saturated_math {
    solana_sdk::declare_id!("HHPC2xVx7S1Lxf2pMK3JySKJZurt5CLpNj6c9LRSyuwz");
}

pub mod check_physical_overlapping {
    solana_sdk::declare_id!("Cyx9FW417wQbNC3GLE9jDuqtUiHVysfUP2siU9cXkFLn");
}

pub mod limit_secp256k1_recovery_id {
    solana_sdk::declare_id!("Dt84qqvdFBVppTdxwTLFtRSBLMfBgbHbxbxEYeX5bpBC");
}

pub mod disable_deprecated_loader {
    solana_sdk::declare_id!("768MU43todZ97n7PnYVmJr8ygUnpHhygaXgWbGmHxFTR");
}

pub mod check_slice_translation_size {
    solana_sdk::declare_id!("FPVk8r4grQQkiwMfcMM2PUjccDWTQCMKKM4cE1ndQsfZ");
}

pub mod stake_split_uses_rent_sysvar {
    solana_sdk::declare_id!("3Fq7Z2QBbuwhQGjeTuN2dNSzRKv38rjQaoVjpZZ9CW62");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    solana_sdk::declare_id!("2JeJcFD1ZNriXcwZegot6ACsbZYqWLKpTYdsLwNEhAkX");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    solana_sdk::declare_id!("9vJtwctZZ1Cd7rdQcLYV8ogxz2KeA4bBkoAf1Yr6kcLE");
}

pub mod reject_callx_r10 {
    solana_sdk::declare_id!("2rDQ1Vm1SSaRmtHThDobnZy79e76br63UUN8Cp6NXZne");
}

pub mod drop_redundant_turbine_path {
    solana_sdk::declare_id!("9EWKPkuSrpx4XXc2UEQrXqHpwmCEjSa3iUkT4GDTAorz");
}

pub mod executables_incur_cpi_data_cost {
    solana_sdk::declare_id!("7utHb3vm2xqUQwEKGF8Kq3CRov8CaPp68hurBNJDFb7C");
}

pub mod fix_recent_blockhashes {
    solana_sdk::declare_id!("CGwzrUUcPSo72h2QViC6UcCNXFv2iM1DkudKW7M3JsM5");
}

pub mod update_rewards_from_cached_accounts {
    solana_sdk::declare_id!("2kdPn3Ey5ECDh3VqpM7EGugJqLDxBoJx6SKPCcoCsEJr");
}
pub mod enable_partitioned_epoch_reward {
    solana_sdk::declare_id!("38g8VJePWcPHYVJDnj14uEvzM3n6vnuao9dvprLC94UM");
}

pub mod spl_token_v3_4_0 {
    solana_sdk::declare_id!("3fMW2T88f8HGUyknyyNWYCVTo95pEvZQTmRMvLFujfBK");
}

pub mod spl_associated_token_account_v1_1_0 {
    solana_sdk::declare_id!("oBYHRgUwkLQM9eZCXzBRTpazWgGJAx1WtHysQaxcQ84");
}

pub mod default_units_per_instruction {
    solana_sdk::declare_id!("79ZUd4LQuuMSWS1Tg84E1RJV9Dd3C2zjqiAsmVrZ94uu");
}

pub mod stake_allow_zero_undelegated_amount {
    solana_sdk::declare_id!("oar3YgmoDYExbEBWEZEjw753eo2NfskRkBcFG5yAuEA");
}

pub mod require_static_program_ids_in_transaction {
    solana_sdk::declare_id!("oGKc7zccts79GHrWwf3ndumuC6FxxCBaoHM3BbqQgBs");
}

pub mod stake_raise_minimum_delegation_to_1_mln {
    // This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    solana_sdk::declare_id!("GDUQPY5WH24nccKtcHtpNMPQp6cP1SSnjePsMHCjEEcs");
}

pub mod stake_minimum_delegation_for_rewards {
    solana_sdk::declare_id!("YcGYaDrMhbhPMoAt5gj4zKGkRYgNndFSXksoyJx8dUR");
}

pub mod add_set_compute_unit_price_ix {
    solana_sdk::declare_id!("4EVwLByjgirSfFFfEFboeK17YGrUBxRXTw3jwdxpDhah");
}

pub mod disable_deploy_of_alloc_free_syscall {
    solana_sdk::declare_id!("5og3u9Vg5sLtUiVC5cdP9PJfrZeRgazzw1zpxGvePLfh");
}

pub mod include_account_index_in_rent_error {
    solana_sdk::declare_id!("5NhMbTH7XQssZHEAJergVacuTnUws6A5ntt524uFpBWx");
}

pub mod add_shred_type_to_shred_seed {
    solana_sdk::declare_id!("AXD9UhsXbQAsAXtBQYJYfrdpQyhZWEtvAAvnm8twWjeg");
}

pub mod warp_timestamp_with_a_vengeance {
    solana_sdk::declare_id!("48WKpfA7e9x2L6o5NkteqvaJs1njoFZHZtH3YL3Avh1X");
}

pub mod separate_nonce_from_blockhash {
    solana_sdk::declare_id!("9xsusmw7MBtYSEkL8essFbQrvr5GdxLSbj1YoqYUKo5J");
}

pub mod enable_durable_nonce {
    solana_sdk::declare_id!("B18WzS8UncGJQzEm772JLbJfYfJ2DfLCqiVgdbYERzCz");
}

pub mod vote_state_update_credit_per_dequeue {
    solana_sdk::declare_id!("3AoE6v4MSmMsBHX7ukSC8xuw4vGccrqpMxbHDEmcmYBD");
}

pub mod quick_bail_on_panic {
    solana_sdk::declare_id!("6Ns8qHyGbv4udAqvhboD9Hn5QLfuT4rXqGE7pU5m28tH");
}

pub mod nonce_must_be_authorized {
    solana_sdk::declare_id!("24kYbqnGtXiyGj23cdWML1gn37SbyQwsi1fmoo8uCExU");
}

pub mod nonce_must_be_advanceable {
    solana_sdk::declare_id!("4fvFevcmNCFH6iivotrNDvCXzLwSNZEnHXNNCZDJBbgu");
}

pub mod vote_authorize_with_seed {
    solana_sdk::declare_id!("GS1diGxAwxQdZxt5WroA6DGc3QgLZzsc2r7Lwd4HTk8H");
}

pub mod cap_accounts_data_size_per_block {
    solana_sdk::declare_id!("FiHSf24BgV6screxfQ8JDNJorfayyTE1WULfWQa5zeEB");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    solana_sdk::declare_id!("2gTPWYyesH2ReNq3d637Ug49UAV4toUU1Rqz3uXcLze5");
}

pub mod enable_bpf_loader_extend_program_ix {
    solana_sdk::declare_id!("CM3tFB8SCkq6b4Y2wpWwoxu7qnW9B5dRT7chof69FYUM");
}

pub mod enable_early_verification_of_account_modifications {
    solana_sdk::declare_id!("BkKuJQrZUByC2TCrC3b4GBnsCDN468w1e8FkMxe9SEZE");
}

pub mod skip_rent_rewrites {
    solana_sdk::declare_id!("5FLf2rN66eUhxtRbRBjGNv6SA8HeYeHJSuZdi9T3gUGp");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    solana_sdk::declare_id!("7JFYPcNdT1AkfDJu9j5uX5toJE1wVvVF3EjopTDRrxM1");
}

pub mod cap_bpf_program_instruction_accounts {
    solana_sdk::declare_id!("E5PVPzbMMBRTVheoCnsfDHdhVt65ByjKwpc6qWjqsgEP");
}

pub mod loosen_cpi_size_restriction {
    solana_sdk::declare_id!("HeMgEi3umaAii72TP2Vd7Q9cjLYihzMs99ZMMVm1Hetc");
}

pub mod use_default_units_in_fee_calculation {
    solana_sdk::declare_id!("FxxuxT6Qa6qGwUhnCJJ9vBYHXa8v8oFbcVMKd7tR92fH");
}

pub mod compact_vote_state_updates {
    solana_sdk::declare_id!("D81C628sHgkTy8bFoSaTFKR8rtFpBgAZpXrwWvjGN2B1");
}

// pub mod concurrent_replay_of_forks {
//     solana_sdk::declare_id!("HKTb558dxjV1pHiUoHY6WiXDW1SPAdR2pp1kqRC74eKy");
// }

pub mod incremental_snapshot_only_incremental_hash_calculation {
    solana_sdk::declare_id!("2ot2vmSkdBsHZzx7192NwSk9vkTdKwU9WudDR8yHXLR4");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    solana_sdk::declare_id!("4BrSEo9yocvW66UXrG6wqzTjiGwRsyq1fKbqk2yrBGaq");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    solana_sdk::declare_id!("5PxkpSkieoAFGaN8NuVXGQnHmVxe27DS5Q42wewroM1w");
}

pub mod account_hash_ignore_slot {
    solana_sdk::declare_id!("Aify2XEM5uFNprngYP8D8jyL1aHJ61FWs5erCxB2aN1r");
}

pub mod set_exempt_rent_epoch_max {
    solana_sdk::declare_id!("7Tr7PC29heiSC7KdhAHCiYHcZdoR2c8cQ3hS7HG4WYC9");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    solana_sdk::declare_id!("C3ntUw4zmc86bNdFnExymqUPG4nUbJgV3Ng5GCPuGgan");
}

pub mod stop_sibling_instruction_search_at_parent {
    solana_sdk::declare_id!("BAAkVzojXVbP8M6UCvSFphu5w4TNkdMK7a8fvXfSg63K");
}

pub mod vote_state_update_root_fix {
    solana_sdk::declare_id!("2NmVW5eyqpz2rk9sJbtpEV9NhQgXPYVWJ5yVo4jp74ai");
}

// pub mod return_none_for_zero_lamport_accounts {
//     solana_sdk::declare_id!("GpdtZ46oewuJTJooRvKQWWWfvmf9SjhfA6kr4BhTTTL6");
// }

pub mod cap_accounts_data_allocations_per_transaction {
    solana_sdk::declare_id!("A4nSusDCkARGEdmhxHfBVKxgqKV79P3hSpBikAio1Ri4");
}

pub mod epoch_accounts_hash {
    solana_sdk::declare_id!("FYiGZ24WhAwRirirZyeKfu1CG7QX455ZN26DhhEz2gxH");
}

pub mod remove_deprecated_request_unit_ix {
    solana_sdk::declare_id!("8Ww4n3KDG4VHfs18EQV1Wh1JJ4XsgZWnmARxYsPNbvV4");
}

pub mod disable_rehash_for_rent_epoch {
    solana_sdk::declare_id!("4ZRYdjX3o3EvWQAftrqCRM3AG3hMvo5Z6TjzMMXJNmLX");
}

pub mod increase_tx_account_lock_limit {
    solana_sdk::declare_id!("5THgob3FTSEkyt2N3qBX2Q8wGMGAmzfpbBhLZoSZKVkK");
}

pub mod limit_max_instruction_trace_length {
    solana_sdk::declare_id!("38srZF58JcmdxFfgnzNPLfmDoodN9pHZsbDLVxWAeCjx");
}

pub mod check_syscall_outputs_do_not_overlap {
    solana_sdk::declare_id!("2Jdyr1T8MLHJo11g1R6x31hQubbiTLTB2PkdWhYpXzVY");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    solana_sdk::declare_id!("Ei1isDj2dmZ2nYrpQjyziauFtogYVSKQKAQvuA6D7zor");
}

pub mod enable_alt_bn128_syscall {
    solana_sdk::declare_id!("A2qmibCwjNqp9ki93c5u2QduQLtorh97QkD3EyHibmgz");
}
pub mod enable_alt_bn128_compression_syscall {
    solana_sdk::declare_id!("3uVo7rt3wpiLCJJ5t6hBVZJRgwLgKjrPcJjRATs6yEFM");
}

pub mod enable_program_redeployment_cooldown {
    solana_sdk::declare_id!("J4pnb9G1wopFrK1PSezsF5MobFVm4wqp1znPrZ8zk1Vg");
}

pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    solana_sdk::declare_id!("7PU1XcsEtKJ3mDEyuEKXhSUapLGM3HMveM97Ghjr6V4u");
}

pub mod enable_turbine_fanout_experiments {
    solana_sdk::declare_id!("9Fi5xXv7RiiKQhrbvwrrM5Q3hEB7VtytegM4U1hWd2ac");
}

pub mod disable_turbine_fanout_experiments {
    solana_sdk::declare_id!("2NowhDaiW3CbPfjzuAdZgyK5fgBpxZyBaQJXHgEmwMbp");
}

// pub mod drop_merkle_shreds {
//     solana_sdk::declare_id!("AAtYRhtYe8RpaVmDYekreCekGaMrd5KycsBTMs28BFbL");
// }

// pub mod keep_merkle_shreds {
//     solana_sdk::declare_id!("3JzPqxHr9evdDxuUfCBB8C9aP5s4X8idQkhoFn3mj5zC");
// }

pub mod move_serialized_len_ptr_in_cpi {
    solana_sdk::declare_id!("D8Uyu9cPJUs4hq3A671SZVb1a245YbdX73H6dmSiyDXe");
}

pub mod update_hashes_per_tick {
    solana_sdk::declare_id!("7FMX8ib6MKWbF1rU22TMQGRLRCS5d8KQRgrq1nsiUeqE");
}

pub mod enable_big_mod_exp_syscall {
    solana_sdk::declare_id!("BABQkLp7BqGxdDZMxKGotaSBJM6vz9mKCCS1yWzfJDn9");
}

pub mod disable_builtin_loader_ownership_chains {
    solana_sdk::declare_id!("Ae1bZsuXGR4WQNgURnitkJ97yzKUnaN9rSZHoC18ep3i");
}

pub mod cap_transaction_accounts_data_size {
    solana_sdk::declare_id!("8bWvzEWb7qvRGeYwkwpEakMAimSv3mNgGjr1Gp84dHXf");
}

pub mod remove_congestion_multiplier_from_fee_calculation {
    solana_sdk::declare_id!("5FpytkUYJPtiiEs348wyKeVswSZkzQrLpcWjobfvvCjp");
}

pub mod enable_request_heap_frame_ix {
    solana_sdk::declare_id!("jGgt7owpUVRmtbbxh4MG6oJTeTVdn8sru7gWrgJWh2W");
}

pub mod prevent_rent_paying_rent_recipients {
    solana_sdk::declare_id!("Dh9AUCEjhCjBSMd2ZWezX1RdzxbrD51RwBFkZ91ptKX6");
}

pub mod delay_visibility_of_program_deployment {
    solana_sdk::declare_id!("9Yenrk5CURWbENBYM4M6YgsP1f5gLFna2J8fgcgUaTbJ");
}

pub mod apply_cost_tracker_during_replay {
    solana_sdk::declare_id!("3kL7bT87XdJJGVwymujE3rDTTLm89zxfjxBYJrERHkja");
}
pub mod bpf_account_data_direct_mapping {
    solana_sdk::declare_id!("FeeggaQwvK34GY3Eep8CGsrkWnJDPf5ixAReZaasZ1hr");
}

pub mod add_set_tx_loaded_accounts_data_size_instruction {
    solana_sdk::declare_id!("G7n7ioWpwAKeRyLA88amhF5AXiV8LKGLK9zpJcpgJPP3");
}

pub mod switch_to_new_elf_parser {
    solana_sdk::declare_id!("AEgCQUhVKm9dHg8Ko8CWnXxbpLxDi8ocxF9YdQUQFRuy");
}

pub mod round_up_heap_size {
    solana_sdk::declare_id!("9gP8EkkdoJs84YirDrr7HDph2eJ2sULfUDEBAyuk6h6f");
}

pub mod remove_bpf_loader_incorrect_program_id {
    solana_sdk::declare_id!("HoGpsuQRozYmcECoQeS2Vko5BUKsspbgustUFAxZKtN8");
}

pub mod include_loaded_accounts_data_size_in_fee_calculation {
    solana_sdk::declare_id!("Em14upxZBEn8phtTU413mciEtTpdjmFgM1r4ZHvgUmoM");
}

pub mod native_programs_consume_cu {
    solana_sdk::declare_id!("AY6myQp4ZBUnm4m6igbjJVyuaoiRbi7vAbmJ8EqtjutR");
}

pub mod simplify_writable_program_account_check {
    solana_sdk::declare_id!("2NeJrnUM8acFXAcdGtrbXQPpRCoUx6yN9fGyzrUbTXyW");
}

pub mod stop_truncating_strings_in_syscalls {
    solana_sdk::declare_id!("Fhuefj6TLhuzpkuuNSN5evJUgeU8ncDqVBrhUaRYzFYJ");
}

pub mod clean_up_delegation_errors {
    solana_sdk::declare_id!("GTwUxeNF4bFhuQZudK7GhhdM6XocZ8tV2oEuRn2tLTL3");
}

pub mod vote_state_add_vote_latency {
    solana_sdk::declare_id!("G5PzC75sXn9xfBs8fYGcPZXkdsEghTxZPtSyoJajg1N");
}

pub mod checked_arithmetic_in_fee_validation {
    solana_sdk::declare_id!("9smpZNhHhNSKk9ofPUF1WDudaRqh1xnWJrZE4A3wgsLm");
}

pub mod last_restart_slot_sysvar {
    solana_sdk::declare_id!("BmRw1Cg72dEdYe6JASpGznodr1ryyLfbFoEPZn8Cha84");
}

pub mod reduce_stake_warmup_cooldown {
    solana_sdk::declare_id!("BkynwF8uZGLaeEFo1ciirN6ozoXtWHdi8AyJvT522zGG");
}

pub mod revise_turbine_epoch_stakes {
    solana_sdk::declare_id!("APLRhyXeMsypAJeBiGyE2uivUi7z8JAnx9gHsY211Pcj");
}

pub mod enable_poseidon_syscall {
    solana_sdk::declare_id!("A2Tg6BQD2btsATMEHwVzT2jXRsueyRyk1jDGME4bsmu8");
}

pub mod timely_vote_credits {
    solana_sdk::declare_id!("76rnHGcyD2bCi5CrgdKjuXknHBjcVhBa3AXD4sBGjj5a");
}

pub mod remaining_compute_units_syscall_enabled {
    solana_sdk::declare_id!("98jLtzmDsy5fRBom4STziC5v4QM7TJtPZuQMxyQC3Anj");
}

pub mod enable_program_runtime_v2_and_loader_v4 {
    solana_sdk::declare_id!("37iUXmu2wwtVGFTkjsKdJozCEDc5NV8SuqKSbRgHRdjN");
}

pub mod require_rent_exempt_split_destination {
    solana_sdk::declare_id!("E88TG7ovEzzjrTcjUs21pGvBJtSJa92vFzHRvqQMAXdd");
}

pub mod better_error_codes_for_tx_lamport_check {
    solana_sdk::declare_id!("FusXtSgpukBFueFMLd6nBcZdnCDAffwUMosw1u9XqLRS");
}

pub mod update_hashes_per_tick2 {
    solana_sdk::declare_id!("8sFZTxpMza4kUoVpvrZz4sk6mcG46BM1d6ghAfd6cz8k");
}

pub mod update_hashes_per_tick3 {
    solana_sdk::declare_id!("8Sh4fvhQnEbULcR4fbnmAf7yjfHzNqrupKM7xXLnt2Qs");
}

pub mod update_hashes_per_tick4 {
    solana_sdk::declare_id!("3JRHWTUhsZdHCyuGvKgMWpLfpQUWkgsXLAz4vqx3u1xu");
}

pub mod update_hashes_per_tick5 {
    solana_sdk::declare_id!("CLXxPbFWLGCVzW1jPo1H38UGwGoz4UncmkrLQsfASZFc");
}

pub mod update_hashes_per_tick6 {
    solana_sdk::declare_id!("FFkv7qUdjindy4vHxDf2SGmmgeNoK4ZVdVWtJKGj4oM4");
}

pub mod validate_fee_collector_account {
    solana_sdk::declare_id!("6rx1ZUWENuBB4og2KqhSCR1cpaFpfjqrLHvQiJfN4VFT");
}

pub mod disable_rent_fees_collection {
    solana_sdk::declare_id!("2m4tqba9mJ2W55Z4tU1QWqsmxxDR1Z4f945TKWooEGuW");
}

pub mod enable_zk_transfer_with_fee {
    solana_sdk::declare_id!("4SfSmAdtkSt5uBGVsx4RBY3Z3jyuo2m2mQss3gA9AvUW");
}

pub mod drop_legacy_shreds {
    solana_sdk::declare_id!("CzP5s8kWixq2U4h7pKeNkzq6x2cVeJhwxgM8bB6m6mCg");
}

pub mod allow_commission_decrease_at_any_time {
    solana_sdk::declare_id!("C7vnTthLpM7Tz5bH5b7My9op4jwoJk4R5PBUCMWRd5dD");
}

pub mod consume_blockstore_duplicate_proofs {
    solana_sdk::declare_id!("sD5CYgZ9acawPhYkpszjQNAphBohP7qYiaEEsewFQaH");
}

pub mod index_erasure_conflict_duplicate_proofs {
    solana_sdk::declare_id!("5xFJJztY9D5EQhLiBthKRHU5kg5h7x1b52vGpnwzYVep");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (secp256k1_program_enabled::id(), "secp256k1 program"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (spl_token_v2_multisig_fix::id(), "solarti-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (spl_token_v2_self_transfer_fix::id(), "solarti-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (blake3_syscall_enabled::id(), "blake3 syscall"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (tx_wide_compute_cap::id(), "transaction wide compute cap"),
        (spl_token_v2_set_authority_fix::id(), "solarti-token set_authority fix"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (disable_fees_sysvar::id(), "disable fees sysvar"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (zk_token_sdk_enabled::id(), "enable Zk Token proof program and syscalls"),
        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
        (versioned_tx_message_enabled::id(), "enable versioned transaction message processing"),
        (libsecp256k1_fail_on_bad_count::id(), "fail libsecp256k1_verify if count appears wrong"),
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsecp256k1_verify if count appears wrong"),
        (instructions_sysvar_owned_by_sysvar::id(), "fix owner for instructions sysvar"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (return_data_syscall_enabled::id(), "enable sol_{set,get}_return_data syscall"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (sol_log_data_syscall_enabled::id(), "enable sol_log_data syscall"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (do_support_realloc::id(), "support account data reallocation"),
        (prevent_calling_precompiles_as_programs::id(), "prevent calling precompiles as programs"),
        (optimize_epoch_boundary_updates::id(), "optimize epoch boundary updates"),
        (remove_native_loader::id(), "remove support for the native loader"),
        (send_to_tpu_vote_port::id(), "send votes to the tpu vote port"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (disable_fee_calculator::id(), "deprecate fee calculator"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (nonce_must_be_writable::id(), "nonce must be writable"),
        (spl_token_v3_3_0_release::id(), "solarti-token v3.3.0 release"),
        (leave_nonce_on_success::id(), "leave nonce as is on success"),
        (reject_empty_instruction_without_program::id(), "fail instructions which have native_loader as program_id directly"),
        (fixed_memcpy_nonoverlapping_check::id(), "use correct check for nonoverlapping regions in memcpy syscall"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        (allow_votes_to_directly_update_vote_state::id(), "enable direct vote state update"),
        (cap_accounts_data_len::id(), "cap the accounts data len"),
        (max_tx_account_locks::id(), "enforce max number of locked accounts per transaction"),
        (require_rent_exempt_accounts::id(), "require all new transaction accounts with data to be rent-exempt"),
        (filter_votes_outside_slot_hashes::id(), "filter vote slots older than the slot hashes history"),
        (update_syscall_base_costs::id(), "update syscall base costs"),
        (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        (vote_withdraw_authority_may_change_authorized_voter::id(), "vote account withdraw authority may change the authorized voter #22521"),
        (spl_associated_token_account_v1_0_4::id(), "SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_transaction_count_fix::id(), "fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
        (disable_bpf_deprecated_load_instructions::id(), "disable ldabs* and ldind* SBF instructions"),
        (disable_bpf_unresolved_symbols_at_runtime::id(), "disable reporting of unresolved SBF symbols at runtime"),
        (record_instruction_in_transaction_context_push::id(), "move the CPI stack overflow check to the end of push"),
        (syscall_saturated_math::id(), "syscalls use saturated math"),
        (check_physical_overlapping::id(), "check physical overlapping regions"),
        (limit_secp256k1_recovery_id::id(), "limit secp256k1 recovery id"),
        (disable_deprecated_loader::id(), "disable the deprecated BPF loader"),
        (check_slice_translation_size::id(), "check size when translating slices"),
        (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        (enable_partitioned_epoch_reward::id(), "enable partitioned rewards at epoch boundary #32166"),
        (spl_token_v3_4_0::id(), "Solarti Token Program version 3.4.0 release #24740"),
        (spl_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (stake_raise_minimum_delegation_to_1_mln::id(), "Raise minimum stake delegation to 1.0 MLN #24357"),
        (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated sol_alloc_free_ syscall"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        // (concurrent_replay_of_forks::id(), "Allow slots from different forks to be replayed concurrently #26465"),
        (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        // (return_none_for_zero_lamport_accounts::id(), "return none for zero lamport accounts #27800"),
        (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        // (drop_merkle_shreds::id(), "drop merkle shreds #29711"),
        // (keep_merkle_shreds::id(), "keep merkle shreds #29711"),
        (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        (remove_congestion_multiplier_from_fee_calculation::id(), "Remove congestion multiplier from transaction fee calculation #29881"),
        (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        (delay_visibility_of_program_deployment::id(), "delay visibility of program upgrades #30085"),
        (apply_cost_tracker_during_replay::id(), "apply cost tracker to blocks during replay #29595"),
        (add_set_tx_loaded_accounts_data_size_instruction::id(), "add compute budget instruction for setting account data size per transaction #30366"),
        (switch_to_new_elf_parser::id(), "switch to new ELF parser #30497"),
        (round_up_heap_size::id(), "round up heap size when calculating heap cost #30679"),
        (remove_bpf_loader_incorrect_program_id::id(), "stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"),
        (include_loaded_accounts_data_size_in_fee_calculation::id(), "include transaction loaded accounts data size in base fee calculation #30657"),
        (native_programs_consume_cu::id(), "Native program should consume compute units #30620"),
        (simplify_writable_program_account_check::id(), "Simplify checks performed for writable upgradeable program accounts #30559"),
        (stop_truncating_strings_in_syscalls::id(), "Stop truncating strings in syscalls #31029"),
        (clean_up_delegation_errors::id(), "Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"),
        (vote_state_add_vote_latency::id(), "replace Lockout with LandedVote (including vote latency) in vote state #31264"),
        (checked_arithmetic_in_fee_validation::id(), "checked arithmetic in fee validation #31273"),
        (bpf_account_data_direct_mapping::id(), "use memory regions to map account data into the rbpf vm instead of copying the data"),
        (last_restart_slot_sysvar::id(), "enable new sysvar last_restart_slot"),
        (reduce_stake_warmup_cooldown::id(), "reduce stake warmup cooldown from 25% to 9%"),
        (revise_turbine_epoch_stakes::id(), "revise turbine epoch stakes"),
        (enable_poseidon_syscall::id(), "Enable Poseidon syscall"),
        (timely_vote_credits::id(), "use timeliness of votes in determining credits to award"),
        (remaining_compute_units_syscall_enabled::id(), "enable the remaining_compute_units syscall"),
        (enable_program_runtime_v2_and_loader_v4::id(), "Enable Program-Runtime-v2 and Loader-v4 #33293"),
        (require_rent_exempt_split_destination::id(), "Require stake split destination account to be rent exempt"),
        (better_error_codes_for_tx_lamport_check::id(), "better error codes for tx lamport check #33353"),
        (enable_alt_bn128_compression_syscall::id(), "add alt_bn128 compression syscalls"),
        (update_hashes_per_tick2::id(), "Update desired hashes per tick to 2.8M"),
        (update_hashes_per_tick3::id(), "Update desired hashes per tick to 4.4M"),
        (update_hashes_per_tick4::id(), "Update desired hashes per tick to 7.6M"),
        (update_hashes_per_tick5::id(), "Update desired hashes per tick to 9.2M"),
        (update_hashes_per_tick6::id(), "Update desired hashes per tick to 10M"),
        (validate_fee_collector_account::id(), "validate fee collector account #33888"),
        (disable_rent_fees_collection::id(), "Disable rent fees collection #33945"),
        (enable_zk_transfer_with_fee::id(), "enable Zk Token proof program transfer with fee"),
        (drop_legacy_shreds::id(), "drops legacy shreds #34328"),
        (allow_commission_decrease_at_any_time::id(), "Allow commission decrease at any time in epoch #33843"),
        (consume_blockstore_duplicate_proofs::id(), "consume duplicate proofs from blockstore in consensus #34372"),
        (index_erasure_conflict_duplicate_proofs::id(), "generate duplicate proofs for index and erasure conflicts #34360"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }

    pub fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch> {
        self.activated_slot(&reduce_stake_warmup_cooldown::id())
            .map(|slot| epoch_schedule.get_epoch(slot))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}

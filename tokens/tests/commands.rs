use {
    miraland_rpc_client::rpc_client::RpcClient,
    miraland_streamer::socket::SocketAddrSpace,
    miraland_test_validator::TestValidator,
    miraland_tokens::commands::test_process_distribute_tokens_with_client,
    solana_sdk::signature::{Keypair, Signer},
};

#[test]
fn test_process_distribute_with_rpc_client() {
    miraland_logger::setup();

    let mint_keypair = Keypair::new();
    let test_validator =
        TestValidator::with_no_fees(mint_keypair.pubkey(), None, SocketAddrSpace::Unspecified);

    let client = RpcClient::new(test_validator.rpc_url());
    test_process_distribute_tokens_with_client(&client, mint_keypair, None);
}

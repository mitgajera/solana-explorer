use solana_client::rpc_client::RpcClient;

pub fn get_rpc_client() -> RpcClient {
    RpcClient::new("https://api.mainnet-beta.solana.com".to_string())
}

pub fn get_devnet_rpc_client() -> RpcClient {
    RpcClient::new("https://api.devnet.solana.com".to_string())
}

pub fn get_testnet_rpc_client() -> RpcClient {
    RpcClient::new("https://api.testnet.solana.com".to_string())
}

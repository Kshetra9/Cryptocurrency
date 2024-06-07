// rust_bitcoin_client/src/main.rs

use bitcoin::blockdata::block::BlockHeader;
use bitcoin::network::constants::Network;
//use bitcoin::util::hash::Sha256dHash;
use bitcoin::hashes::sha256d::Hash as Sha256dHash;
use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    let rpc = Client::new(
        "http://localhost:8332",
        Auth::UserPass("test_user".to_string(), "password".to_string()),
    )
     .unwrap();

    match rpc.get_block_count() {
        Ok(block_count) => println!("Block count: {}", block_count),
        Err(e) => eprintln!("Error getting block count: {:?}", e),
    }
}
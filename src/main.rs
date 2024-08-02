use std::sync::Arc;

use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{contract::SierraClass, BlockId, BlockTag, Felt},
    },
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // Sierra class artifact. Output of the `starknet-compile` command
    let contract_artifact: SierraClass =
        serde_json::from_reader(std::fs::File::open(
            "/home/ubuntu/test_cairo/erc20/target/dev/erc20_NFT.contract_class.json").unwrap())
            .unwrap();

    // Class hash of the compiled CASM class from the `starknet-sierra-compile` command
    let compiled_class_hash = Felt::from_hex("0x003a9fc523cb4f18df04808df092abe7a1614616829cc9de0988c63322314bfb").unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x027e41147490a7913cf56bafdc381795595532852a2fe8071b917869eca2d38a").unwrap(),
    ));
    let address = Felt::from_hex("0x059e0eaf58972c3b7de923ad6a280476430295f7ea967b768bd381bf5d90d50b").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    // account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // We need to flatten the ABI into a string first
    let flattened_class = contract_artifact.flatten().unwrap();

    let result = account
        .declare_v2(Arc::new(flattened_class), compiled_class_hash)
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
    println!("Class hash: {:#064x}", result.class_hash);
}
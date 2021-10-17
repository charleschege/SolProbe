mod simplified_block;
pub use simplified_block::*;
mod solana_blocks;
pub use solana_blocks::TransactionError;
mod endpoints;
pub use endpoints::*;

#[test]
fn decode() {
    use crate::solana_blocks::BlockResponse;
    use json::{array, object};
    smol::block_on(async {
        let request_data = object! {
            jsonrpc: "2.0",
            id: 1,
            method: RpcRequest::GetBlock.to_camel_case(),
            params: array![1]
        };

        let mut request = surf::post(SOLANA_MAINNET_BETA_URL)
            .header("Content-Type", "application/json")
            .body_string(request_data.to_string())
            .await
            .unwrap();

        let response = request.body_string().await.unwrap();

        let jd = &mut serde_json::Deserializer::from_str(&response);

        let parsed_response: BlockResponse = serde_path_to_error::deserialize(jd).unwrap();
        let mut end = ConfirmedBlock::new();
        end.decode(&parsed_response.result);

        println!("{:#?}", &end);
    })
}

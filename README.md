#### SolProbe

This crate simplifies the data structure returned by Solana JSON RPC and makes it easy for Solana Dapp developers to access a simplified data structure rather than using the Solana Library that has very deep nested structures. 

This crate was created after having to destructure deeply nested data structures from the Solana JSON API response. I needed a way to make this easier and reuse the code for different Dapps. 

This crate is also Web Assembly friendly since it is lean and it depends on very little dependencies unlike the official Solana crate that are huge and have tons of dependencies some of which panic when compiled to Web Assembly.

There is a caveat to this, the part of the Solana program instructions has been left out since Dapp developers most of the time do not need to decode instructions. Furthermore, the encoded format used to encode these instructions is not available for reference unless a developer knows them before hand. If you feel you need this you can just used the official Solana Program Library, or open an issue or pull request in this repo.

#### Crate Documentation is missing FOR NOW :(

##### LICENSE

This repository is licensed under Apache-2.0 and uses parts of the Solana Program Library which is also licensed  under Apache-2.0. All contributions will be under the same license.

Happy Dapp hacking :)



##### Example usage

```rust
use crate::solana_blocks::BlockResponse;
    use json::{array, object};

// Use any method to get the JSON data from Solana RPC node
// Here, `smol` crate is used
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

        // Decode the data using serde_json since we are fetching JSON
        let parsed_response = serde_json::from_str(&response).unwrap();
        // Use this library to transform the reponse into
        //Developer friendly data structures
        let mut end_result = ConfirmedBlock::new();
        end_result.decode(&parsed_response.result);

        println!("{:#?}", &end);
    })
```


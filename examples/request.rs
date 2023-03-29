#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let TENANT_ACCESS_TOKEN_URI = "/open-apis/auth/v3/tenant_access_token/internal";
    let url = format!("{}{}", "https://open.feishu.cn", TENANT_ACCESS_TOKEN_URI);

    let res: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&serde_json::json!({
            "app_id": "cli_a37e9201cb3bd00c",
            "app_secret": "loUnBxUAABMFJKHn0kXm5l2nVoJ8dE3s",
        }
        ))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", res);
    println!(
        "tenant_access_token: {}",
        res["tenant_access_token"].to_string()
    );

    Ok(())
}

// //! This example illustrates the way to send and receive arbitrary JSON.
// //!
// //! This is useful for some ad-hoc experiments and situations when you don't
// //! really care about the structure of the JSON and just need to display it or
// //! process it at runtime.

// // This is using the `tokio` runtime. You'll need the following dependency:
// //
// // `tokio = { version = "1", features = ["full"] }`
// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let echo_json: serde_json::Value = reqwest::Client::new()
//         .post("https://jsonplaceholder.typicode.com/posts")
//         .json(&serde_json::json!({
//             "title": "Reqwest.rs",
//             "body": "https://docs.rs/reqwest",
//             "userId": 1
//         }))
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("{:#?}", echo_json);
//     // Object(
//     //     {
//     //         "body": String(
//     //             "https://docs.rs/reqwest"
//     //         ),
//     //         "id": Number(
//     //             101
//     //         ),
//     //         "title": String(
//     //             "Reqwest.rs"
//     //         ),
//     //         "userId": Number(
//     //             1
//     //         )
//     //     }
//     // )
//     Ok(())
// }

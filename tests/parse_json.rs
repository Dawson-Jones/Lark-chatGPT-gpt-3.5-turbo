use feishu_gptbot::feishu::ReceiveMessage;

#[test]
fn parse_json() {
    let jsonify = r#"
    {
        "schema": "2.0",
        "header": {
            "event_id": "22cf33a32dc1be8be7b42b077266950f",
            "token": "1p37CgbHAxiVCmzPhY8epfT2rV7YBSQi",
            "create_time": "1677774685550",
            "event_type": "im.message.receive_v1",
            "tenant_key": "108fa0df1a45d740",
            "app_id": "cli_a37e9201cb3bd00c"
        },
        "event": {
            "message": {
                "chat_id": "oc_91c8cdcdc087efc5f3b813e8daca6850",
                "chat_type": "p2p",
                "content": "{\"text\":\"hi\"}",
                "create_time": "1677774685358",
                "message_id": "om_4514953cbffa6c6469bf9f1f5ed025c9",
                "message_type": "text"
            },
            "sender": {
                "sender_id": {
                    "open_id": "ou_2fdd4ad406f66b80b797704e3ab31e71",
                    "union_id": "on_eb267e617a2e46c25f513baea736f857",
                    "user_id": "eada3d15"
                },
                "sender_type": "user",
                "tenant_key": "108fa0df1a45d740"
            }
        }
    }"#;

    let obj: ReceiveMessage = serde_json::from_str(jsonify).unwrap();
    println!("{:#?}", obj);
}

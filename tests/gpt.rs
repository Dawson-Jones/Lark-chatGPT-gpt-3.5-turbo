use std::io::{self, Write};

use feishu_gptbot::chatgpt::request_for_chatgpt;

#[tokio::test]
async fn t() {
    for _ in 0..8 {
        print!("Q: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let a = request_for_chatgpt("dawson".to_string(), user_input)
            .await
            .unwrap();

        println!("{}", a);
        // assert!(!a.is_empty());
    }
}

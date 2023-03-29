use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    name: String,
    age: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

fn main() {
    // let string = r#"{"Request":{"id":"1234","method":"POST","params":{"name":"dawson","age":"12"}}}"#;
    // let msg: Message = serde_json::from_str(string).unwrap();
    // println!("{:#?}", msg);

    let string = r#"
    { 
        "Response": { 
            "id": "123", 
            "result": {
                "code": 200 
            } 
        } 
    }
    "#;
    let msg: Message = serde_json::from_str(string).unwrap();
    println!("{:#?}", msg);

    let msg = Message::Request {
        id: "1234".to_string(),
        method: "POST".to_string(),
        params: Params {
            name: "dawson".to_string(),
            age: "12".to_string(),
        },
    };
    println!("msg: {:#?}", msg);
    let jsonify = serde_json::to_string(&msg).unwrap();
    println!("{}", jsonify);

    let msg = Message::Response {
        id: "1123".to_string(),
        result: Value { code: 200 },
    };
    let jsonify = serde_json::to_string(&msg).unwrap();
    println!("{}", jsonify);
}

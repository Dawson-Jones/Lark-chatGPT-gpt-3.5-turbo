use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    req: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    resp: Option<String>,
}

fn main() {
    let point = Point {
        x: 1,
        y: 2,
        req: "hello".to_string(),
        resp: None,
    };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:#?}", deserialized);
}

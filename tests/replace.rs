#[test]
fn t() {
    let my_string = "hello, world!";
    let unwanted_substring = "lo";
    let new_string = my_string.replace(unwanted_substring, "");
    println!("{}", new_string);
    assert!(new_string == "hel, world!");
}

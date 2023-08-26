pub fn print_hello() {
    let msg: String = serde_json::from_str("hello").unwrap();
    println!("{msg}");
}

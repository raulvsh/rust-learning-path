fn main() {
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
}

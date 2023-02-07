fn main() {
    let mut s = String::from("hello world");

    let tmp_s = s.clone();
    let word = first_word(&tmp_s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..4]
}
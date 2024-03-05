fn main() {
    let hello = String::from("中国人");
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
    // println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

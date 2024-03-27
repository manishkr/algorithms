fn first_word(s: &str) -> Option<&str> {
    let first_word = s.split_whitespace().next();
    if let Some(word) = first_word {
        return Some(word);
    }
    None
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{:?}", word);

    let word = first_word(&my_string[..]);
    println!("{:?}", word);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{:?}", word);
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{:?}", word);

    let word = first_word(&my_string_literal[..]);
    println!("{:?}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{:?}", word);

    let x = &my_string[0..6];
    println!("{}", x);

    let x: &str = &my_string[..];
    println!("{}", x);
}

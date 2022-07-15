fn main() {
    string_slice();
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..2];
    println!("a={:?} slice={:?}", a, slice);
}

fn string_slice() {
    let mut s = String::from("Hello world!");
    let hello = &s[..5];
    let world = &s[6..];
    let hw = &s[..];

    let idx = first_word(&s);
    println!("idx = {idx}, {hello}, {world}, {hw}");
    s.clear();
    // println!("idx={idx}"); will error due to mutable borrow on line above
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, char) in bytes.iter().enumerate() {
        if *char == b' ' {
            return &s[..i];
        }
    }
    s
}

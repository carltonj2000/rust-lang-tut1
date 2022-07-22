use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    vector();
    strings();
    hashmaps();
}

fn hashmaps() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // println!("{blue}"); // errors due to ownership transfer
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    println!("{:?}", scores.get(&team_name));

    for (k, v) in &scores {
        println!("k {k}, v {v}");
    }

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 60);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(200);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn strings() {
    let s1 = String::new();
    let s2 = "initial content";
    let s3 = s2.to_string();
    let s4 = String::from("initial content");

    println!("{s1} {s3} {s4}");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{s}");

    let s5 = s3 + &" ".to_string() + &s4;
    //println!("{s1}"); // err due to ownership moving to s3
    println!("{s5}");
    println!("{}", format!("{}{}", s5, '!'.to_string()));
    println!("{s5}"); // ownership not transferred with format

    // let c = s3[0]; // err could be byte, scalar, grapheme
    for b in s2.bytes() {
        println!("{b}");
    }
    for c in s2.chars() {
        println!("{c}");
    }
    println!("graphemes");
    for g in s2.graphemes(true) {
        println!("{g}");
    }
}

fn vector() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut v2 = vec![1, 2, 3];
    println!("a = {:?}, v = {:?}, v2 = {:?}", a, v, v2);

    {
        let third = &v[2];
        // let twenty = &v[20]; // run time error
        println!("third = {:?}", third);
    }

    {
        let idx = 2;
        match v.get(idx) {
            Some(nth) => println!("element {} = {:?}", idx + 1, nth),
            None => println!("There is no {}th element", idx),
        }
    }
    {
        #[allow(unused_variables)]
        let third = &v2[2];
        v2.push(4);
        // println!("third={}", third); // err due to mut borrow
        println!("v = {:?}", v2);
    }
    {
        for i in &mut v2 {
            *i += 100;
        }
        for i in v2 {
            println!("{:?}", i);
        }
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Non int"),
    }
}

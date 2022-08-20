use std::fmt::Display;

fn main() {
    lt1();
    lt2();
    lt3();
}

fn lt1() {
    let s1 = String::from("cj1");
    {
        let s2 = String::from("cj2 2");
        let r = longest(s1.as_str(), s2.as_str());
        println!("Longest {}", r);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn get_part(&self, ann: &str) -> &str {
        println!("Attention: {}", ann);
        self.part
    }
}

fn lt2() {
    let novel = String::from("Call me Ishmael. Some year ago ...");
    let first_s = novel.split('.').next().expect("Could not find first sentence");
    let i =  ImportantExcerpt {
        part: first_s,
    };
    println!("part {}", i.get_part(String::from("hi").as_str()));
}

fn lt3() {
    let x = String::from("str x");
    let y = String::from("str y2");
    let lwaa = longest_with_an_announcement(x.as_str(), y.as_str(), "hi there");
    println!("lwaa {}", lwaa);
}

fn longest_with_an_announcement<'a, T>(
   x: &'a str,
   y: &'a str,
   ann: T,
    ) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {x} else {y}
}

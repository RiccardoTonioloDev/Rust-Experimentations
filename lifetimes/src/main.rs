use std::fmt::Display;

fn main() {
    let r; //1
    {
        let x = 5; //2
        r = &x; //3
        println!("r: {}", r); //5
    } //4
    println!("tra ciao e bau la più lunga è {}", longest("ciao", "bau"));

    let string1 = String::from("Long long string");
    let result;
    let first;
    {
        let string2 = String::from("little");
        result = longest(string1.as_str(), string2.as_str());
        first = first_string(string1.as_str(), string2.as_str());
    }
    //THIS WON'T COMPILE
    //println!("The longhest string is {}",result);
    println!("The first string is {}", first);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_string<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
    //In entrambi i casi viene applicata l'elision rule
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

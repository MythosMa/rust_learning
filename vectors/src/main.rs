use std::collections::HashMap;
fn main() {
    // let v = vec![1, 2, 3, 4, 5, 6];
    // let third = &v[2];

    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    let mut s1 = String::from("hello");
    let s2 = " world!";
    s1.push_str(s2);
    println!("s2 is now {}", s2);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // panic!("Panic!");

    let v = vec![1, 2, 3, 4, 5];
    v[99];
}

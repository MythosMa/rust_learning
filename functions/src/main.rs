fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(56, 'F');
    let x = five();
    println!("The value of x is: {}", x);
    let y = plus_one(5);
    println!("The value of y is: {}", y);

    let s1 = String::from("hello");
    task_ownership(s1);
    // println!("{}", s1);

    let mut s2 = String::from("hello");
    changeStr(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    let rs1 = &s3;
    let rs2 = &s3;
    println!("{}, {}", rs1, rs2);
    println!("{}", rs1);
    let rs3 = &mut s3;
    println!("{}", rs3);
}

fn another_function() {
    println!("This is another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn task_ownership(str: String) {
    println!("{}", str);
}

fn changeStr(str: &mut String) {
    str.push_str(" after change!");
}

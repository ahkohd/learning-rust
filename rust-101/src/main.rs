fn main() {
    const FIRST_NAME: &str = "Victor";
    let mut last_name = String::new();
    last_name.push_str("Aremu");

    let full_name = FIRST_NAME.to_string() + &last_name;
    println!("{} {} {}", FIRST_NAME, last_name, full_name);

    let str_: &str = "hello";
    let x: u8 = 12;
    let ranges: (u8, &str) = (12, "hello");

    let yes = match ranges.0 {
        12 => true,
        _ => false,
    };

    println!("{} {:b} {} {}", str_, x, ranges.1, yes);

    let arr: [&str; 4] = ["1", "2", "3", "4"];
    let arr_slice: &[&str] = &arr[0..2];

    print!("{} {}", arr[0], arr_slice[1]);

    let tuple: (&str, u32) = ("hello", 12);
    print!("{} {}", tuple.0, tuple.1);

    print!("{}", sum(&12, &13));

    let (a, b, c) = (1, 2, 3);
    print!("{} {} {}", a, b, c);

    do_some_math();
    do_some_logical_ops();
    comparison_ops();
    bitwise_ops()
}

fn sum(x: &i32, y: &i32) -> i32 {
    return x + y;
}

fn do_some_math() {
    let mut a = 4;
    let mut b = 3;
    a = a + b;
    a = a * b;
    a = a - b;
    b = b - a;
    println!("a:{}", a);
    println!("b:{}", b);
}

fn do_some_logical_ops() {
    let mut a = false;
    let mut b = true;
    a = a && b || (!a);
    b = !b;
    println!("a:{}", a);
    println!("b:{}", b);
}

fn comparison_ops() {
    let mut a = true;
    let mut b = true;
    a = a > b && b < a;
    b = !b;
    println!("a: {}", a);
    println!("b: {}", b);
}

fn bitwise_ops() {
    const MSG: &str = "Some bitwise ops";
    println!("{}", MSG);

    let a = 2;
    let b = 3;

    println!("{:b} & {:b} = {:b}", a, b, a & b);
    println!("{:b} | {:b} = {:b}", a, b, a | b);
    println!("{:b} ^ {:b} = {:b}", a, b, a ^ b);
    println!("!{:b} = {:b}", a, !a);
    println!("Left shift {:b} = {:b}", a, a << 2);
    println!("Right shift {:b} = {:b}", 11, 11 >> 2);
}

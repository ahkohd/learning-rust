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
    bitwise_ops();
    learn_borrowing();
    learning_conditionals();
    learning_simple_calculator();
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

fn learn_borrowing() {
    // Borrowing means to reference the original data binding or to share the data.
    // Borrowing is a way to share data without having to clone it.

    // Two variables are involved in a borrowing relationship when the referenced variable holds a value that the referencing variable borrows.
    // The referencing variable simply points to the memory location of the referenced variable.

    // Shared borrowing: A piece of data that is shared by single or multiple variables but it cannot be altered
    let a = 10;
    let b: &i32 = &a;
    println!("a:{} b:{}", a, b);

    // Mutable borrowing: A piece of data that is shared and altered by a single variable (but the data is inaccessible to other variables at that time)
    let mut c = 10;
    let d = &mut c;
    println!("d:{}", d);

    *d = 20; // dereference
    println!("d:{}", d);
    println!("c:{}", c);

    // There can be either one mutable borrow or any number of immutable borrows within the same scope.
}

fn learning_conditionals() {
    let age = 12;
    // conditinal ternary operator
    let is_old = if age > 18 { true } else { false };
    println!("is {}yrs old enough? {}", age, is_old);

    let langs = ("Rust", "Javascript", "PHP");

    if let ("Rust", "Javascript", _) = langs {
        println!("Javascript is one of the languages!");
    } else {
        println!("Not found!")
    }

    // match
    let lang = match langs.1 {
        "Javascript" => "Love!",
        "Rust" => "Crabs",
        "PHP" => "Elephant!",
        _ => "Not found!",
    };

    println!("{}", lang);

    const A: i32 = 4;
    println!(
        "Number {} is {}",
        A,
        if A % 2 == 0 { "even" } else { "odd" }
    )
}

fn learning_simple_calculator() {
    macro_rules! to_str {
        ($a:expr) => {{
            $a.to_string()
        }};
    }

    fn calc(a: i32, operator: char, b: i32) {
        let exp: String = match operator {
            '+' => to_str!(a + b),
            '-' => to_str!(a - b),
            '*' => to_str!(a * b),
            '%' => to_str!(a % b),
            '/' => {
                if b != 0 {
                    to_str!(a / b)
                } else {
                    String::from("Division by 0 is undefined")
                }
            }
            _ => String::from("invalid operator"),
        };

        println!("{}", exp);
    }

    calc(3, '+', 2);
    calc(3, '-', 2);
    calc(3, '*', 2);
    calc(3, '/', 2);
    calc(3, '%', 2);
    calc(3, '(', 2);
    calc(3, '/', 0);
}

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
    learning_loops();
    learning_functions();
    learning_strings();

    let msg = "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course".to_string();
    println!("{}", concatenate_words_starting_with_c(msg));

    learning_vectors();
    learning_sructs();
    learning_enums();
    learn_option_enums();
    learn_result_enums();
    learning_traits();
    learn_generics();
}

fn sum(x: &i32, y: &i32) -> i32 {
    x + y
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

fn learning_loops() {
    for i in 0..10 {
        println!("loop: {}", i);
    }

    let arr: [i32; 2] = [1, 2];

    for i in arr.iter() {
        println!("traverse: {} ", i);
    }

    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    let mut x: i8 = 1;
    while x < 10 {
        println!("while: {}", x);
        x += 1;
    }
}

fn learning_functions() {
    // To be clear, Rust does not have "implicit returns" nor does it use semicolons to indicate return values.
    // Expressions evaluate to a value, and semicolons take an expression, throw away its value, and evaluate to () instead. That's it.
    // The other behaviors you're talking about fall out of these semantics, but are also different than what you've said;
    // functions evaluate to a value, so the final expression's value determines the value it evaluates to, but you cannot "implicitly return" from the middle of a function by dropping a ;, for example.

    // pass by value
    let mut a = 10;
    let mut b = 2;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    println!("a+b={}", add(a, b));

    // this next line actually works, because it's a premitive type.
    // if it was an object, it would not work because it now has a new ownership in the function add, therefore
    // a & b now longer contains their original values.
    println!("a:{} b:{}", a, b);

    // pass by reference
    fn add_ref(a: &mut i32, b: &mut i32) -> i32 {
        *a = 20;
        *b = 30;
        *a + *b
    }

    println!("a+b={}", add_ref(&mut a, &mut b));
    println!("a:{} b:{}", a, b);

    // multiple return values
    // calculate area and perimeter
    fn calculate_area_perimeter(x: i32, y: i32) -> (i32, i32) {
        // calculate the area and perimeter of rectangle
        let area = x * y;
        let perimeter = 2 * (x + y);
        // return the area and perimeter of rectangle
        (area, perimeter)
    }

    let (area, perimeter) = calculate_area_perimeter(10, 20);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}

fn learning_strings() {
    // an empty string
    let course1 = String::new();
    let s_course1 = course1.to_string();
    // print the String object
    println!("This is an empty string {}.", s_course1);
    // print the length of an empty String  object
    println!("This is a length of my empty string {}.", s_course1.len());

    // create a String literal
    let course2: &str = "Rust Programming";
    // convert String literal to string object using .to_string
    let s_course2 = course2.to_string();

    // print the String object
    println!("This is a string literal : {}.", s_course2);
    // print the length of a String object
    println!("This is a length of my string literal {}.", s_course2.len());

    // define a String object using from method
    let course3 = String::from("Rust Language");
    // print the String object
    println!("This is a string object : {}.", course3);
    // print the length of an string object
    println!("This is the length of my string object {}.", course3.len());

    for found in course2.chars() {
        println!("{}", found);
    }

    // updating strings...
    // define a String object
    let mut course = String::from("Rus");
    // push a character
    course.push('t');
    println!("This is a beginner course in {}.", course);

    // define a string object
    let mut course = String::from("Rust");
    // push a string
    course.push_str(" Programming");
    println!("This is a beginner course in {}.", course);

    // concatenate two strings
    // define a String object
    let course = "Rust".to_string();
    // define a String object
    let course_type = " beginner course".to_string();
    // concatenate using the + operator
    let result = course + &course_type;
    println!("{}", result);

    // format marcro - adds two or more String objects together
    let course = "Rust".to_string();
    let _course_type = "beginner course".to_string();
    // default format macro
    let result = format!("{} {}", course, _course_type);
    // passing value in the placeholder in the format macro
    let _result = format!("{1} {0}", course, _course_type);
    println!("{}", result);
    println!("{}", _result);

    // slicing strings
    let string = "Rust Programming".to_string();
    let slice = &string[5..12];
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Slice : {}", slice);
}

fn concatenate_words_starting_with_c(my_str: String) -> String {
    let mut str = "".to_string();
    let words = my_str.split_whitespace();
    for word in words {
        if word.starts_with("c") {
            str.push_str(word);
            str.push(' ');
        }
    }

    str
}

// @note
// tuples, arrays, and slices, are fixed length.
// vectors are variable length arrays.

fn learning_vectors() {
    let programs = vec!["Rust", "Programming", "Language"];
    let mut my_vec = Vec::new();
    println!("{:?} {}", programs, programs[1]);

    match programs.get(4) {
        Some(value) => print!("{}", value),
        None => println!("Not found!"),
    }

    for program in &programs {
        println!("{}", program);
    }

    for (index, value) in programs.iter().enumerate() {
        println!("Element at index {}:{} ", index, value);
    }

    let index = programs
        .iter()
        .position(|&r| r == "Programming")
        .unwrap_or(12);

    println!("Index of Programming: {}", index);

    println!("Empty Vector : {:?}", my_vec);
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
    my_vec.pop();
    println!("Popped value: {}", 3);
    println!("Popped element at last index : {:?}", my_vec);
    my_vec.remove(1);
    println!("Removed value: {}", 2);
    println!("Removed element at index 1 : {:?}", my_vec);
    println!("Size of vector is :{}", my_vec.len());
    println!("Does my vector contains 1 : {}", my_vec.contains(&1));

    // define a vector of size 5
    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("Initial Vector : {:?}", my_vec);
    for x in my_vec.iter_mut() {
        *x *= 3;
    }
    // print the updated vector
    println!("Updated Vector : {:?}", my_vec);

    let slice: &[i32] = &my_vec[2..4];
    // print the vector
    println!("Slice of the vector : {:?}", slice);

    my_vec.remove(my_vec.len() / 2);
    my_vec.pop();
    my_vec.push(my_vec.iter().sum());

    println!("Result: {:?}", my_vec);
}

fn learning_sructs() {
    // define a struct
    struct Course {
        code: i32,
        name: String,
        level: String,
    }

    let mut course1 = Course {
        name: String::from("Rust"),
        level: String::from("beginner"),
        code: 130,
    };

    let course2 = Course {
        name: "Javascript".to_string(),
        level: "beginner".to_string(),
        code: 122,
    };

    //access
    println!(
        "Name:{}, Level:{}, code: {}",
        course1.name, course1.level, course1.code
    );
    println!(
        "Name:{}, Level:{}, code: {}",
        course2.name, course2.level, course2.code
    );
    //update
    course1.name = "Java".to_string();
    course1.code = 134;
    println!(
        "Name:{}, Level:{} ,code: {}",
        course1.name, course1.level, course1.code
    );

    //methods
    // The method is like a regular function except that the &self
    // parameter is passed to it and the items within the function
    // are accessed through it.

    // impl keyword is used to implement a trait for a struct, to define
    // struct methods.

    //impl construct to define struct methods
    impl Course {
        fn name_code(&self) -> String {
            format!("{} {}", self.name, self.code)
        }

        // static method
        fn my_static_method(n: String, l: String, c: i32) -> Course {
            Course {
                name: n,
                level: l,
                code: c,
            }

            // this is a static method because it does not use the self
        }

        fn display(&self) {
            println!(
                "name :{} code:{} of type: {}",
                self.name, self.code, self.level
            );
        }
    }

    let course_1 = Course {
        name: "Rust".to_string(),
        level: "beginner".to_string(),
        code: 132,
    };

    println!(
        "This is a {} course: {}",
        course_1.level,
        course_1.name_code()
    );

    let c1 = Course::my_static_method("Rust".to_string(), "beginner".to_string(), 132);
    c1.display();

    // tuple struct
    // tuples can be of type struct by adding the struct keyword before
    // the tuple name, followed by the data type of the variables enclosed within round brackets.

    //define a tuple struct
    struct FruitQuantity(String, i32);

    // create an instance
    let r1 = FruitQuantity("oranges".to_string(), 12);
    println!("r1--name:{} quantity:{}", r1.0, r1.1);
    // create an instance
    let r2 = FruitQuantity("mangoes".to_string(), 13);
    // access values of a tuple struct
    println!("r2--name:{} quantity:{}", r2.0, r2.1);

    // example
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn distance_to(&self, point2: Point) -> f32 {
            let result = (&self.x - point2.x).pow(2) + (&self.y - point2.y).pow(2);
            (result as f32).sqrt()
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 9, y: 2 };

    print!("Distance between p1 and p2 is {}", p1.distance_to(p2));
}

fn learning_enums() {
    // define an enum

    // make this `enum` printable with `fmt::Debug`.
    #[derive(Debug)]
    enum KnightMove {
        Horizontal,
        Vertical,
    }

    // use enum
    let horizontal_move = KnightMove::Horizontal;
    let vertical_move = KnightMove::Vertical;

    // print the enum values
    println!("Move 1: {:?}", horizontal_move);
    println!("Move 2: {:?}", vertical_move);

    // enums with variants
    // make this `enum` printable with `fmt::Debug`.
    #[derive(Debug)]
    enum KnightMove2 {
        Horizontal(String),
        Vertical(String),
    }

    // invoke an enum
    let horizontal_move = KnightMove2::Horizontal("Left".to_string());
    let vertical_move = KnightMove2::Vertical("Down".to_string());
    // print enum
    println!("Move 1: {:?}", horizontal_move);
    println!("Movw 2: {:?}", vertical_move);

    // enums and methods
    #[derive(Debug)]
    // declare an enum
    enum TrafficSignal {
        Red,
        Green,
        Yellow,
    }
    //implement a Traffic Signal methods
    impl TrafficSignal {
        // if the signal is red then return
        fn is_stop(&self) -> bool {
            match self {
                TrafficSignal::Red => return true,
                _ => return false,
            }
        }
    }

    // define an enum instance
    let action = TrafficSignal::Red;
    //print the value of action
    println!("What is the signal value? - {:?}", action);
    //invoke the enum method 'is_stop' and print the value
    println!("Do we have to stop at signal? - {}", action.is_stop());

    // @notes
    // in rust enums are not limited to only one variant
    // rust match keyword is used to match the value of an enum, and the matching must be exhaustive.
}

fn learn_option_enums() {
    // Option is a built-in enum in the Rust standard library. It has two variants Some and None.
    // Variants:
    //   Some(T), returns Some value T
    //   None, returns no value

    fn learn_lang(my_lang: &str) -> Option<bool> {
        if my_lang == "Rust" {
            Some(true)
        } else {
            None
        }
    }

    println!("{:?}", learn_lang("Rust"));
    println!("{:?}", learn_lang("Python"));

    match learn_lang("Rust") {
        Some(x) => println!("Yelp! {}", x),
        None => println!("Lang not rust!"),
    }

    // Example 2: Optional Variable Value# optinal variable value
    struct Course {
        code: i32,
        name: String,
        level: Option<String>,
    }

    let course1 = Course {
        name: String::from("Rust"),
        level: Some(String::from("beginner")),
        code: 130,
    };
    let course2 = Course {
        name: String::from("Javascript"),
        level: None,
        code: 122,
    };

    println!(
        "Name:{}, Level:{} ,code: {}",
        course1.name,
        course1.level.unwrap_or("Level".to_string()),
        course1.code
    );
    println!(
        "Name:{}, Level:{} ,code: {}",
        course2.name,
        course2.level.unwrap_or("No level defined!".to_string()),
        course2.code
    );

    // Example 3: Index Out of Bound Exception

    // define a variable
    let str = String::from("Educative");
    // define the index value to be found
    lookup(str, 12);

    fn lookup(str: String, index: usize) {
        let matched_index = match str.chars().nth(index) {
            // execute if match found print the value at specified index
            Some(c) => c.to_string(),
            // execute if value not found
            None => "No character at given index".to_string(),
        };
        println!("{}", matched_index);
    }

    // is_some(), is_none() Functions
    fn print(my_val: Option<&str>) {
        if my_val.is_some() {
            // check if the value is equal to some value
            println!("my_val is equal to some value");
        } else {
            println!("my_val is equal to none");
        }
    }

    let my_val: Option<&str> = Some("Rust Programming!");
    let my_val2: Option<&str> = None;
    print(my_val); // invoke the function
    print(my_val2); // invoke the function
}

fn learn_result_enums() {
    use rand::Rng;

    // Result is a built-in enum in the Rust standard library. It has two variants Ok(T) and Err(E).
    // Variants:
    // Ok(T), returns the success statement of type T
    // Err, returns the error statement of type E.

    fn file_exists() -> Result<bool, bool> {
        return if rand::thread_rng().gen_bool(1.0 / 2.0) {
            Ok(true)
        } else {
            Err(false)
        };
    }

    match file_exists() {
        Ok(x) => println!("File exists! {}", x),
        Err(x) => println!("File not found! {}", x),
    }

    fn divisible_by_3(i: i32) -> Result<String, String> {
        if i % 3 == 0 {
            // if number mod 3 equals 0
            Ok("Given number is divisible by 3".to_string()) // return this statement
        } else {
            // if if number mod 3 is not equals 0
            Err("Given number is not divisible by 3".to_string()) // return this statement
        }
    }

    println!("{:?}", divisible_by_3(6)); // invoke function by passing a number 6
    println!("{:?}", divisible_by_3(2));

    // is_ok(), is_err() Functions

    let check1 = divisible_by_3(6);
    if check1.is_ok() {
        // check if the function returns ok
        println!("The number is divisible by 3");
    } else {
        println!("The number is not divisible by 3");
    }

    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> i32 {
            match self {
                Days::Saturday | Days::Sunday => 1,
                _ => 0,
            }
        }
    }

    println!("{}", Days::Saturday.is_weekend());
}

fn learning_traits() {
    // Traits are a way to define behavior for a type.
    // They are like interfaces but not limited to interfaces.
    // There can be two types of methods in traits
    // Concrete Method
    // The method that has a body meaning that implementation of the method is done within the method.
    //
    // Abstract Method
    // The method that does not have a body meaning that implementation of the method is done by each struct in its own impl construct.

    //declare a structure
    struct Circle {
        radius: f32,
    }
    struct Rectangle {
        width: f32,
        height: f32,
    }

    //declare a trait
    trait Area {
        fn shape_area(&self) -> f32;
    }

    //implement the trait for the structure
    impl Area for Circle {
        fn shape_area(&self) -> f32 {
            3.13 * self.radius * self.radius
        }
    }

    impl Area for Rectangle {
        fn shape_area(&self) -> f32 {
            self.width * self.height
        }
    }

    //create an instance of the structure
    let c = Circle { radius: 2.0 };
    let r = Rectangle {
        width: 2.0,
        height: 2.0,
    };
    println!("Area of Circle: {}", c.shape_area());
    println!("Area of Rectangle:{}", r.shape_area());
}

fn learn_generics() {
    fn test_generic<T>(x: T) -> T {
        x
    }

    println!("{}", test_generic::<i32>(10));

    struct Rect<T> {
        width: T,
        height: T,
    }

    let r: Rect<f32> = Rect {
        width: 10.0,
        height: 20.0,
    };
}

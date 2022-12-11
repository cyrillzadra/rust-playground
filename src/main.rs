/**
println!() is a macro - used in metaprogramming, i.e., code that writes code. expanded into source that gets compiled.

Build-in macros -> https://doc.rust-lang.org/std/#macros


 */

fn main() {
    println!("Hello World!");

    named_arguments();
    placeholder_traits();
    getting_started_debug_trait();
    getting_started_challenge_1();
    getting_started_challenge_2();

    variable_initialize();
    variable_mutable();
    variable_scope();
    variable_multiple();
    variable_shadowing();

    variable_challenge_1();


    data_type_types();

    operators_borrowing_dereferencing();
    conditional_expressions();

    loops();
    functions();
}

fn named_arguments() {
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
}

fn placeholder_traits() {
    println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}", 10, 10, 10);
}

fn getting_started_debug_trait() {
    println!("{:?}", ("This is a Rust Course", 101));
}

/**
print macros

print!(), println!()
eprint!(), eprintln!()  - prints output as an error

 */

fn getting_started_challenge_1() {
    print!("I am learning Rust programming language");
}

fn getting_started_challenge_2() {
    println!("1");
    println!("22");
    println!("333");
    println!("4444");
    println!("55555");
}

fn variable_initialize() {
    let language = "Rust"; // language is immutable
    println!("Language: {}", language);
}

fn variable_mutable() {
    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable
}


fn variable_multiple() {
    let (course, category) = ("Rust", "Beginner");
    println!("This is a {} course in {}.", category, course); // print the value
}


fn variable_scope() {
    //local & global
    let inner = 1;
    {
        //let inner = 1; // local
    }
    println!("{}", inner);
}

fn variable_shadowing() {
    let outer = 12;
    {
        let inner = 1;
        println!("{}", inner);
        let outer = 9;
        println!("{}", outer);
    }
    println!("{}", outer);
}

fn variable_challenge_1() {
    let mut x = 1000;
    let y = "Programming";
    println!("x:{}", x);
    println!("y:{}", y);
    x = x + 100;
    println!("x:{}", x);
    println!("y:{}", y);
}


fn data_type_types() {
    //scalar type - integer, string, boolean, character
    //compound type = array, tuple

    let a: i8 = 21;
    let b: i16 = 1;
    let c = 54;
    let d = 343434;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);


    let s = 'T';
    let ss = "Test";
    println!("{}", s);
    println!("{}", ss);

    let arr: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", arr);

    //define the slice
    let slice_array1: &[i32] = &arr;
    let slice_array2: &[i32] = &arr[0..2];

    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);


    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    println!("{:?}", person_data);
    // define a tuple with type annotated
    let person_data: (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
    println!("{:?}", person_data);

    println!("{} , {} , {}, {}", person_data.0, person_data.1, person_data.2, person_data.3);


    //challenge

    let arr: [i32; 6] = [0, 2, 4, 6, 8, 10];
    println!("{},{},{},{},{},{}", arr[0], arr[1], arr[2], arr[3], arr[4], arr[5]);

    //Alex:21, Abe:22, Anna:23
    let t1 = ("Alex", 21);
    let t2 = ("Abe", 22);
    let t3 = ("Anna", 23);


    let tuples = (t1, t2, t3);

    println!("{:?}", tuples);
    println!("{}:{}, {}:{}, {}:{}", t1.0, t1.1, t2.0, t2.1, t3.0, t3.1);

    let b = true;
    let bb = false;
    println!("{}", b && bb);
}

fn operators_borrowing_dereferencing() {
    //References are just like pointers in C.


    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed

    //mutable reference to a variable
    let b = &mut y;
    println!("Value of b:{}", b);

    *b = 11; // derefencing
    println!("Value of b:{}", b); // updated value of b
    println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed


    //mutable reference to a variable
    let mut x = 10;
    println!("Value of x:{}", x);
    let a = &mut x;
    println!("Value of a:{}", a);
    //dereference a variable
    *a = 11;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // Note that value of x is updated

    let a = 2;
    let b = 2;
    // Write code here!
    println!("{}", i32::pow(a + b, 3));
}


fn conditional_expressions() {
    let learn_language = "Rust";

    if learn_language == "Rust" {
        println!("You are learning Rust language!");
    } else { println!("Learn Rust!") }


    // define a scrutinee expression
    let course = ("Rust", "beginner", "course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", "course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // define a variable
    let x = 5;
    // define match expression
    match x {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C++"),
        4 => println!("C#"),
        5 => println!("Rust"),
        6 => println!("Kotlin"),
        _ => println!("Some other value"),
    };
}

fn loops() {
    for i in 0..5 {
        println!("{}", i);
    }

    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    let mut var = 1; //define an integer variable
    let mut found = false; // define a boolean variable
    // define a while loop
    while !found {
        var = var + 1;
        //print the variable
        println!("{}", var);
        // if the modulus of variable is 1 then found is equal to true
        if var % 2 == 1 {
            found = true;
        }
        println!("Loop runs");
    }


    for i in 1..5 { //outer loop
        println!("Multiplication Table of : {}", i);
        for j in 1..5 { // inner loop
            println!("{} * {} = {}", i, j, i * j);
        }
    }

    'outer: for i in 1..5 { //outer loop
        println!("Muliplication Table : {}", i);
        'inner: for j in 1..5 { // inner loop
            if i == 3 { continue 'outer; } // Continues the loop over `i`.
            if j == 2 { continue 'inner; } // Continues the loop over `j`.
            println!("{} * {} = {}", i, j, i * j);
        }
    }
    let n = 3;
    let mut f = 1;
    for i in 1..n + 1 {
        f *= i;
    }
    println!("{}", f);


    let mut n = 30;
    let mut c = 0;
    // define a while loop
    while n >= 0 {
        n -= 3;
        c += 1;
    }
    println!("{}", c);

    let b = 5;
    for i in 1..b + 1 { //outer loop
        for j in 0..i { // inner loop
            print!("&");
        }
        println!("");
    }
}

fn functions() {
    let n = 4;
    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    fn_pass_by_value(n);
    println!("\nThe value of n after function call : {}", n);

    let mut n = 4;
    println!("The value of n before function call : {}", n);
    println!("Invoke Function");
    fn_pass_by_ref(&mut n);
    println!("The value of n after function call : {}", n);
}

fn fn_pass_by_value(mut n: i32) {
    n = n * n;
    println!("The value of n inside function : {}", n);
}

fn fn_pass_by_ref(n: &mut i32) {
    *n = *n * *n;
    println!("The value of n inside function : {}", n);
}

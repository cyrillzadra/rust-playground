use std::marker::Tuple;

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
}

fn named_arguments() {
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
}

fn placeholder_traits() {
    println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}", 10, 10, 10);
}

fn getting_started_debug_trait() {
    println!("{:?}", ("This is a Rust Course",  101));
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
        let outer = 9;
        println!("{}", outer);
    }
    println!("{}", outer);
}

fn variable_challenge_1() {
    let mut x = 1000;
    let y = "Programming";
    println!("x:{}" , x);
    println!("y:{}" , y);
    x = x + 100;
    println!("x:{}" , x);
    println!("y:{}" , y);
}


fn data_type_types() {

    /**
    scalar type - integer, string, boolean, character
    compound type = array, tuple

    let scalar_type : integer =  10;

    println!("{}", scalar_type) */

}

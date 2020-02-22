use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // Recoverable Errors
    // File::open returns a Result enum
    let f = File::open("hello.txt");
    // we can use unwrap() or expect()
    // If there is an error it will panic
    // let f = File::open("hello.txt").unwrap()
    // if there is an error it will be catched
    // let f = File::open("hello.txt").expect("Error message goes here")

    // It is possbile to recover from and error which occured
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Unrecoverable Errors

    let v = vec![1, 2, 3];

    // Trying to access index out of bound will make the programm to panic
    //v[99];

    // Next commad will lead the programm to crush with a message provided
    // to the panic! macro.
    // To enable backtrace you can run a programm with RUST_BACKTRACE=1 cargo run
    //panic!("crashed")
}

// This function is propagating the error to caller
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // <-- there is no ;
}

// Same function as above, but it has shorthand
// ? can be use in functions that return Result
fn read_username_from_file_with_shorhand() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    //It could be chained as well
    // let mut s = String::new()
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // Or it could be one line only using following function, but
    // we won't be able to explain to a user an error in details
    // fs::read_to_string("hello.txt")
}

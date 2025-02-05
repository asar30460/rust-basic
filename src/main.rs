use rand::Rng;
use std::io::Write; // For utilizing io::stdout().flush()
use std::{cmp::Ordering, io};

fn main() {
    // mut means mutable. If not specified, the variable is immutable. But mut is something different from const, which would be elaborated later
    let mut option = String::new();
    cli_out_options();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    // str.trim() would remove leading and trailing whitespace
    match option.trim() {
        "1" => cmp_num(true),
        "2" => const_mut_shadowing(),
        "3" => control_flow(),
        "4" => enum_struct(),
        _ => {
            // default
            let mut rng = rand::rng();
            let secret_number = rng.random_range(1..101);
            println!(
                "Undefined but show a random number for you: {}",
                secret_number
            );
        }
    }
}

fn cli_out_options() {
    println!(
        "Welcome to Rust playground. Author: Tyler\n========================================="
    );
    println!("1. Compare number");
    println!("2. Const, Mut and Shadowing");
    println!("3. Control Flow");
    println!("4. Enum and Struct");

    print!("=========================================\nEnter option: ");

    /*
     * Rust automatically flushes the output buffer when a newline is printed, so println!() outputs immediately.
     * Without a newline, text from print!() remains in the buffer and won't be displayed.
     * This means print!() output may appear delayed until the buffer is flushed (e.g., by user input).
     * Call io::stdout().flush() after print!() to force immediate output.
     */
    io::stdout().flush().expect("Failed to flush stdout"); // 強制立即輸出緩衝區
}

fn cmp_num(looping: bool) {
    if looping {
        // The loop keyword creates an infinite loop.
        loop {
            // Quitting after a good score
            if execute() {
                break;
            }
        }
    } else {
        execute();
    }

    fn execute() -> bool {
        let mut score: String = String::new();
        print!("Enter score: ");
        io::stdout().flush().expect("Failed to flush stdout"); // 強制立即輸出緩衝區

        io::stdin()
            .read_line(&mut score)
            .expect("Failed to read line");

        // The way to convert a string to a number. But wait, doesn’t the program already have a variable named score?
        // Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the score variable
        let score: u64 = score.trim().parse().expect("Failed to parse");

        match score.cmp(&60) {
            Ordering::Less => println!("Failed"),
            Ordering::Greater => {
                println!("Good");
                return true;
            }
            Ordering::Equal => println!("Safe"),
        }

        false // Most functions return the last expression implicitly. Be sure not to use a semicolon after the return keyword.
    }
}

fn const_mut_shadowing() {
    /*
     * const variables is initialize at compile time, while mutable variables is initialize at runtime
     * const variables cannot be changed forever, while mutable variables can be changed by mut
     * const variables cannot be shadowed
     */
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // underscore means ignore declaration but not use

    /*
     * Shared Reference is allowed to read data but not modify it.
     * Since it can be only read, there can exist multiple shared references and it is safe to pass between threads.
     *
     * Unique Reference is allowed to read and modify data.
     * Since it can be both read and modify, there can exist only one unique reference.
     * This exclusivity prevents data races and ensures that when you’re modifying data.
     */

    let mut x = 42;

    let _shared_ref = &x;

    let unique_ref = &mut x;
    *unique_ref += 1; // modify directly

    /*
     * Shadowing spares us from having to come up with different names (e.g. str_total).
     * The Second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
     */

    let x = 5; // let make variable redeclare
    let x = x + 1; // x = 6 until the scope ends

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Shadowing make second variable change type
    let spaces = "   "; // Bound to a string literal (&str). Note that value not can be mutable since the key concept of shadowing is to reuse the variable name
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}

fn control_flow() {
    // if in let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {result}");

    // Loop labels in nested loops with prefix '
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // for-loop range
    for x in 0..3 {
        // The ".." generates numbers from a start number up to but not including an end number.
        // loop through [0, 3)
        print!("{x} ");
    }
    println!();

    for x in 0..=3 {
        // The "=.." generates numbers from a start number up to and including an end number.
        // loop through [0, 3]
        print!("{x} ");
    }
    println!();
}

fn enum_struct() {
    // Define an enum for V6 format with two variants.
    enum V6Format {
        Bin,
        Hex,
    }

    // Define an enum for IP address kind that uses V6Format.
    enum IpAddrKind {
        V4,
        V6(V6Format),
        None,
    }

    // Define a struct that holds various data, including an IpAddrKind.
    struct Ap {
        id: i32,
        name: String,
        kind: IpAddrKind,
        address: String,
    }

    let aps = vec![
        Ap {
            id: 1,
            name: String::from("AC-01"),
            kind: IpAddrKind::V4,
            address: "127.0.0.1".to_string(),
        },
        Ap {
            id: 2,
            name: String::from("ACX-01"),
            kind: IpAddrKind::V6(V6Format::Hex),
            address: "::1".to_string(),
        },
    ];

    for ap in aps {
        match ap.kind {
            IpAddrKind::V4 => println!(
                "id: {}, name: {}, kind: IPv4, address: {}",
                ap.id, ap.name, ap.address
            ),
            IpAddrKind::V6(V6Format::Hex) => println!(
                "id: {}, name: {}, kind: IPv6 (Hex), address: {}",
                ap.id, ap.name, ap.address
            ),
            IpAddrKind::V6(V6Format::Bin) => println!(
                "id: {}, name: {}, kind: IPv6 (Bin), address: {}",
                ap.id, ap.name, ap.address
            ),
            IpAddrKind::None => println!(
                "id: {}, name: {}, kind: None, address: {}",
                ap.id, ap.name, ap.address
            ),
        }
    }
}


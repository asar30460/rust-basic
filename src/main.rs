use rand::Rng;
use std::io::Write; // For utilizing io::stdout().flush()
use std::{cmp::Ordering, io};

fn main() {
    choose_fn();
    cmp_num(true);
    const_mut_shadowing();
}

fn choose_fn() {
    // mut means mutable. If not specified, the variable is immutable. But mut is something different from const, which would be elaborated later
    let mut option = String::new();

    print!("Enter option: ");

    /*
     * Rust automatically flushes the output buffer when a newline is printed, so println!() outputs immediately.
     * Without a newline, text from print!() remains in the buffer and won't be displayed.
     * This means print!() output may appear delayed until the buffer is flushed (e.g., by user input).
     * Call io::stdout().flush() after print!() to force immediate output.
     */
    io::stdout().flush().expect("Failed to flush stdout"); // 強制立即輸出緩衝區

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..101);

    match option.trim() {
        // str.trim() would remove leading and trailing whitespace
        "1" => println!("Run first function to get random number: {}", secret_number),
        _ => println!("Undefined"),
    }
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
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /*
     * Shared Reference is allowed to read data but not modify it.
     * Since it can be only read, there can exist multiple shared references and it is safe to pass between threads.
     *
     * Unique Reference is allowed to read and modify data.
     * Since it can be only read and modify, there can exist only one unique reference.
     * This exclusivity prevents data races and ensures that when you’re modifying data.
     */

    let mut x = 42;

    let _shared_ref = &x; // underscore means ignore declaration but not use

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

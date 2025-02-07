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
        "5" => generic_type(),
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
    println!("5. Generic Type");

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
    // Enumerations allow you to create a new type that can have a value of several tagged elements
    enum V6Format {
        Bin,
        Hex,
    }

    enum IpAddrKind {
        V4,
        V6(V6Format),
        None, // None represents the absensce of a kind
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
            address: "2377:3b49:3ef1:63ab:b003:8a2e:5b4c:3d02".to_string(),
        },
        Ap {
            id: 3,
            name: String::from("ACX-01"),
            kind: IpAddrKind::V6(V6Format::Bin),
            address: "2377:3b49:3ef1:63ab:b003:8a2e:5b4c:3d02".to_string(),
        },
        Ap {
            id: 4,
            name: String::from("AC-02 --beta"),
            kind: IpAddrKind::None,
            address: "0.0.0.0".to_string(),
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
                ap.id,
                ap.name,
                hex_to_bin(&ap.address)
            ),
            IpAddrKind::None => println!("error fetch IP address kind"),
        }
    }

    fn hex_to_bin(hex: &String) -> String {
        let mut bin = String::new();
        for c in hex.chars() {
            if c == ':' {
                bin.push_str(":");
                continue;
            }
            bin.push_str(&format!("{:04b}", c.to_digit(16).unwrap()));
        }
        bin
    }
}

fn generic_type() {
    /*
     * Generic types allow us to partially define a struct or enum,
     * enabling a compiler to create a fully defined version at compile-time based off our code usage.
     * Rust generally can infer the final type by looking at our instantiation,
     * but if it needs help you can always be explicit using the ::<T> operator, also known by the name turbofish.
     */

    struct ReqData<T> {
        data: T,
    }

    let number_data = ReqData { data: 42 };
    let string_data = ReqData {
        data: "42".to_string(),
    };

    println!("{}, {}", number_data.data, string_data.data);

    /*
     * Unlike many languages that use null or nil to represent the absence of a value—often leading to runtime errors if mishandled—Rust uses the Option<T> enum.
     * This enum has two variants: Some(T) for a present value and None for the absence of a value.
     * The compiler forces you to explicitly handle both cases, reducing the risk of runtime errors associated with null references."
     *
     * Below shows some common real-world examples of using Option<T> in Rust:
     */

    // --- Searching in a Collection ---
    let numbers = vec![1, 2, 3, 4, 5];
    let result = numbers.iter().find(|&&x| x > 3);

    match result {
        Some(&value) => println!("Found a number greater than 3: {}", value),
        None => println!("No number greater than 3 found."),
    }
    // === End of searching in a Collection ===

    // --- Parsing a String ---
    let number_str = "42";
    let number: Option<i32> = number_str.parse::<i32>().ok();

    match number {
        Some(n) => println!("Parsed number: {}", n),
        None => println!("Failed to parse the number."),
    }
    // === End of parsing a String ===

    // --- Optional Configuration or Parameters ---
    struct Config {
        port: Option<u16>, // Some API fields may be optional
        host: String,
    }

    let config = Config {
        port: Some(8080),
        host: String::from("localhost"),
    };

    if let Some(port) = config.port {
        println!("Server will run on {}:{}", config.host, port);
    } else {
        println!("Using default port.");
    }
    // === End of Optional Configuration or Parameters ===

    // -- Returning Early from a Function
    fn find_user_by_id(id: i32) -> Option<String> {
        if id == 1 {
            return Some("Alice".to_string());
        }

        None
    }

    println!("{:?}", find_user_by_id(2)); // :? is debug format specifier and Option<T> implement std::fmt::Debug

    // Use match statement to handle Some and None cases separately
    match find_user_by_id(2) {
        Some(user) => println!("{user}"),
        None => println!("No user found"),
    }
    // === End of Returning Early from a Function ===

    /*
     * Rust has another useful generic enum called Result<T, E> that is often used for error handling.
     * This enum is so common, instances of the enum can be created anywhere with the enum variants Ok and Err.
     */

    pub fn max_profit(prices: Vec<i32>) -> Result<i32, String> {
        if prices.len() < 2 {
            return Err(String::from("Not enough data to calculate profit"))
        }

        let mut left = 0;
        let mut profit = 0;

        for right in 1..prices.len()-1 {
            let temp = prices[right] - prices[left];
            if temp > 0 {
                // profit = cmp::max(profit, temp);
                profit = profit.max(temp) // Rust’s inherent methods.
            } else {
                left = right
            }
        }

        Ok(profit)
    }

    let nv_stock = vec![123, 114, 118, 116, 143, 128];
    match max_profit(nv_stock) {
        Ok(v) => println!("Max Profit: {}", v),
        Err(e) => println!("err: {}", e),
    }

    // Result is so common that Rust has a powerful operator ? for working with them. The following statement is equivalent to above:
    // But it only works in functions that return a type like Result, Option, or any other type that implements.
    // let result = max_profit(nv_stock)?;
    // println!("Max Profit by ? operator: {}", result);


    
}

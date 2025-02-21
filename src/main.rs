use rand::Rng;
use std::io::Write;
use std::time::Duration;
use std::{str, vec};
// For utilizing io::stdout().flush()
use std::{cmp::Ordering, error::Error, fmt::Display, io, ops::Deref};

use std::thread;

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
        "4" => closures(),
        "5" => enum_struct(),
        "6" => generic_type(),
        "7" => ownership_and_borrowing(),
        "8" => text(),
        "9" => oop(),
        "10" => smart_pointers(),
        "11" => tokio_async_programming(),
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

    let menu_optrions = vec![
        "Compare number",
        "Const, Mut and Shadowing",
        "Control Flow",
        "Closures",
        "Enum and Struct",
        "Generic Type",
        "Ownership and Borrowing",
        "Text",
        "OOP",
        "Smart Pointers",
        "Tokio Asynchronous Programming",
    ];
    for (id, option) in menu_optrions.iter().enumerate() {
        println!("{}", &format!("{:>2}: {}", id + 1, option)); // :>2 indicates right alignment, and 2 sets the width to 2 characters
    }

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

fn closures() {
    /*
     * Closures in Rust are anonymous functions that can capture variables from their surrounding environment
     * That is, by defining a closure with lambdas, to represent some parameters are passed into a function
     * The syntax is: |parameters| expression. Below shows some common uses of closures in Rust:
     */

    // || indicates that the closure takes no parameters.
    let greeting = || println!("Hello, world!");
    greeting();

    // |x| defines a closure that takes one argument x
    let square = |x| x * x;
    let result = square(5);
    println!("5 squared is {}", result);

    // |x, y| defines a closure that takes two arguments x and y.
    let add = |x, y| x + y;
    let result = add(5, 3);
    println!("5 + 3 is {}", result);

    // Ignore Parameters (|_|). The closure |_, message| takes two parameters, but only message is used.
    let print_message = |_, message| println!("Message: {}", message);
    print_message(42, "Hello!");
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
            return Err(String::from("Not enough data to calculate profit"));
        }

        let mut left = 0;
        let mut profit = 0;

        for right in 1..prices.len() - 1 {
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
    //
    // let result = max_profit(nv_stock)?;
    // println!("Max Profit by ? operator: {}", result);

    /*
     * Vec is a collection type which is a variably sized list of items.
     * The macro vec! lets us easily create a vector rather than manually constructing one.
     * Vec has the method iter() which creates an iterator from a vector.
     */

    // Rust infers type automatically
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Explicit type with turbofish
    let mut v2 = Vec::<f32>::new();
    v2.push(4.1);
    v2.push(4.2);
    v2.push(4.3);

    // Concatenate vectors
    let v3 = vec![String::from("a"), "b".to_string()];
    for (idx, val) in v3.iter().enumerate() {
        println!("The {idx}th element in vector is: {val}");
    }

    /*
     * Eager vs. Lazy Evaluation and extend implementation
     *
     * In Rust, evaluation strategy refers to "when and how expressions are computed".
     *
     * Eager Evaluation: Expressions are evaluated immediately, regardless of whether their results are ultimately used.
     * Lazy Evaluation: Expressions are evaluated only when their results are needed.
     */

    // unwrap
    {
        /*
         * Description: Extracts the value if the Result is Ok or the Option is Some. If it's an Err or None, it panics.
         * Evaluation: No additional values are provided, so there's no evaluation of a default or alternative value.
         */
        let x: Result<u32, &str> = Ok(10);
        assert_eq!(x.unwrap(), 10);
    }

    // For later paradigm usage
    fn expensive_computation() -> u32 {
        println!("expensive_computation");
        42
    }

    // unwrap_or
    {
        /*
         * Description: Returns the contained value (Ok or Some) or a provided default if it’s an Err or None.
         * Eagerly Evaluated: The default value is always computed, regardless of whether it's needed.
         */
        let default = expensive_computation();
        let x: Result<u32, &str> = Err("error");
        assert_eq!(x.unwrap_or(default), default);
    }

    // unwrap_or_else
    {
        /*
         * Description: Similar to unwrap_or, but instead of taking a value, it takes a closure (a function) that produces the default value.
         * Lazily Evaluated: The closure is only executed if the Result is Err or the Option is None.
         */
        let x: Result<u32, u32> = Err(404);
        assert_eq!(
            x.unwrap_or_else(|err| {
                println!("error: {}", err);
                err
            }),
            expensive_computation()
        );
    }
}

fn ownership_and_borrowing() {
    #[derive(Copy, Clone)] // Make struct Foo implement copy trait for later demo of deference
    struct Foo {
        x: i32,
    }

    {
        // Instantiating a type and binding it to a variable name creates a memory resource that the Rust compiler will validate through its whole lifetime.
        // The bound variable is called the resource's owner.
        let _foo = Foo { x: 42 }; // Creating memory resource and _foo is the owner

        // Rust uses the end of scope as the place to deconstruct and deallocate a resource.
        // The term for this deconstruction and deallocation is called a drop.
        // Memory detail: Rust does have GC, It instead using paradigm Resource Acquisition Is Initialization ( RAII ) in C++.

        // *** _foo is dropped here ***
    }

    {
        // When an owner is passed as an argument to a function, ownership is moved to the function parameter.
        // After a move the variable in the original function can no longer be used.

        fn do_something(f: Foo) {
            println!("{}", f.x);
            // f is dropped here
        }

        let foo = Foo { x: 42 };
        // foo is moved to do_something
        do_something(foo);
        // foo can no longer be used
    }

    {
        // Returning Ownership

        fn do_something() -> Foo {
            Foo { x: 42 } // ownership is moved out
        }

        let _foo = do_something(); // _foo becomes the owner
    }

    {
        // References allow us borrow access to a resource with the & operator.
        let foo = Foo { x: 42 };
        let bar = &foo;

        println!("Get foo after being borrowed: {}", foo.x);
        println!("The var who borrowed from foo: {}", bar.x);
    }

    {
        // Borrowing Mutable Ownership with References
        // Rust prevents having two ways to mutate an owned value because it introduces the possibility of a data race.

        let mut foo = Foo { x: 42 };
        let bar = &mut foo;

        // foo.x = 43; // FAILURE: A resource owner cannot be moved or modified while mutably borrowed.
        bar.x = 43; // bar is dropped here because it's no longer used after this point
        foo.x = 43; // This works now because all mutable references were dropped
    }

    {
        // Dereferencing (When you want to access the data/value in the memory that the pointer points to - the contents of the address with that numerical index - then you dereference the pointer.)
        // Using &mut references, you can set the owner's value using the * operator.
        // You can also get a copy of an owned value using the * operator (if the value can be copied - we will discuss copyable types in later chapters).

        let mut foo = Foo { x: 42 };
        let bar = &mut foo;
        let baz = *bar; // Get a copy of the owner's value (Since Foo implements the Copy trait, this operation copies the value of foo)
        *bar = Foo { x: 43 }; // Set the reference's owner's value

        println!("baz.x is still {} since it was copied before change", baz.x);
        println!("foo.x, however, is now updated to {}", foo.x);
    }

    {
        // Passing around Borrowed Data
        // Rust only allow there to be one mutable reference of a multiple non-mutable references but not both.
        // A reference must never live longer than its owner (Like preventing from dangling pointers in C).

        fn do_something(f: &mut Foo) {
            f.x += 1;
        }

        let mut foo = Foo { x: 42 };
        do_something(&mut foo);

        // foo is dropped here
    }

    {
        // Explicit Lifetimes
        // Rust compiler knows lifetime of each variable and will attempt to validate that a reference never exists longer than its owner.

        /*
         * The <'a> after the function name introduces a lifetime parameter named 'a.
         * Both foo and bar are references to Foo that must live at least as long as 'a.
         * The return type is a reference to an i32 that is also guaranteed to be valid for the lifetime 'a.
         * This tells the Rust compiler that the returned reference will not outlive the data referenced by foo or bar.
         */
        fn bigger_one<'a>(foo: &'a Foo, bar: &'a Foo) -> &'a i32 {
            // 'a represents some span of time during which the references are valid
            if foo.x > bar.x {
                &foo.x
            } else {
                &bar.x
            }
        }

        let foo = Foo { x: 42 };
        let bar = Foo { x: 43 };
        println!("bigger one is: {}", bigger_one(&foo, &bar));

        /*
         * Consider a struct that holds a reference:
         * The ImportantExcerpt returned by get_excerpt can only be used for as long as the original String exists.
         * Once the original String goes out of scope, the ImportantExcerpt is no longer valid.
         */
        struct ImportantExcerpt<'a> {
            part: &'a str, // part is valid for some lifetime 'a.
        }
        fn get_excerpt<'a>(s: &'a String) -> ImportantExcerpt<'a> {
            // The lifetime 'a ensures that the returned excerpt does not outlive the original string.
            ImportantExcerpt {
                part: &s[0..s.len() / 2],
            }
        }

        // A correct usage
        {
            let foo = String::from("foo");
            let excerpt = get_excerpt(&foo);
            println!("Excerpt: {}", excerpt.part);
        }

        // An error demonstrating lifetime
        // let excerpt = {
        //     let foo = String::from("foo");
        //     get_excerpt(&foo)
        // };
        // println!("Excerpt: {}", excerpt.part);
    }
}

fn text() {
    /*
     * String literals are always Unicode and its type are &'static str where
     * "'static" meaning the string data is created at compile time and will be available till the end of our program (it never drops)
     * "str" means that it points to a sequence of bytes that are always valid utf-8
     */

    let a: &'static str = "hello, 🦀";
    println!("text: {}, len: {}", a, a.len());

    // Some useful string methods
    let mut s = String::from("hello");
    s.push_str(" world!");
    println!("{}", s.to_uppercase());
    println!("{}", &["hello", " ", "world", "!"].concat()); // hello world!. The temporary String lives until the end of the println!
    println!("{}", ["a", "b", "c"].join(",")); // a,b,c. The & takes a reference to this temporary String
}

fn oop() {
    struct ChatCompletionMessage {
        role: String,
        content: String,
        last_response: Option<String>,
    }

    // Rust supports the concept of an object that is a struct associated with some functions (also known as methods).
    impl ChatCompletionMessage {
        // The first parameter of any method must be a reference to the instance associated with the method call
        fn show_input_tokens(&mut self) {
            // &self - Immutable ref to the inst; &mut self Mutable one
            self.role = self.role.to_uppercase();
            println!("{}: {}", &self.role, &self.content);
        }

        // By default fields and methods are accessible only to the module they belong to. Use pub to make them public
        pub fn show_response(&mut self) {
            let response = format!(
                "{} seems to wanna know {}. There are some advices...",
                self.role, self.content
            );
            println!("{}", response);
            self.last_response = Some(response);
        }
    }

    let mut new_chat = ChatCompletionMessage {
        role: "devloper".to_string(),
        content: "Explain OOP in Rust".to_string(),
        last_response: None,
    };

    new_chat.show_input_tokens();
    new_chat.show_response();

    // Polymorphism provides ability for objects of different types through a common interface.
    // Rust supports polymorphism with traits. Traits allow us to associate a set of methods with a struct type.
    trait Token {
        fn compute_tokens(&self) -> i32;

        // Traits can have implemented methods.
        fn use_multi_trait_fns(&self) {
            let a = self.compute_tokens();
            let b = self.compute_tokens();

            println!("If you ask 2 times, you will consume {} tokens", a + b);
        }
    }

    impl Token for ChatCompletionMessage {
        fn compute_tokens(&self) -> i32 {
            let input_tokens: i32 = self.content.len() as i32;

            let output_tokens: i32;
            match &self.last_response {
                Some(response) => output_tokens = response.len() as i32,
                None => output_tokens = 0,
            }

            input_tokens + output_tokens
        }
    }

    println!(
        "Total comsumed tokens in this conversation: {}",
        new_chat.compute_tokens()
    );
    new_chat.use_multi_trait_fns();

    // Traits can inherit methods from other traits.
    trait ConversationToken: Token {
        fn compute_conversation_tokens(&self) {
            let a = self.compute_tokens();
            let b = self.compute_tokens();

            println!(
                "If you ask 2 times in same conversation, you will consume {} tokens",
                (1 + 2) * 2 / 2 * (a + b)
            );
        }

        // You can also implement methods outside of the trait
        fn compute_conversation_tokens_with_lite_model(&self);
    }

    // Don't forget to implement the trait
    impl ConversationToken for ChatCompletionMessage {
        fn compute_conversation_tokens_with_lite_model(&self) {
            let lite_model_discont = 0.5;
            let tokens = self.compute_tokens() as f32 * 2.0 * lite_model_discont;

            println!(
                "If you ask 2 times in same conversation with lite model, you will consume {} tokens",
                tokens
            );
        }
    }
    new_chat.compute_conversation_tokens();
    new_chat.compute_conversation_tokens_with_lite_model();

    /*
     * One trait can be implemented by multiple structs.
     * However, If you call a function which passes a reference to a trait, How Rust know which struct to call the function on?
     * Trait types &dyn MyTrait give us the ability to work with instances of objects indirectly using dynamic dispatch.
     * When dynamic dispatch is used, Rust will encourage you to put dyn before your trait type so people are aware.
     * Dynamic dispatch is slightly slower because of the pointer chasing to find the real function call.
     */

    fn dynamic_dispatch(conversation_token: &dyn ConversationToken) {
        print!("[Dynamic dispatch mode] ");
        io::stdout().flush().expect("Failed to flush stdout");
        conversation_token.compute_conversation_tokens();
    }
    dynamic_dispatch(&new_chat);

    /*
     * Traits introduce an interesting challenge when we want to store them within another struct.
     * Traits obfuscate the original struct thus it also obfuscates the original size.
     * Unsized values being stored in structs are handled in two ways in Rust:
     *
     * generics - Using parameterized types effectively create struct/functions known types and thus known sizes.
     * indirection - Putting instances on the heap gives us a level of indirection that allow us to not have to worry about the size of the actual type and just store a pointer to it. There are other ways as well!
     */

    // Generics
    fn generic_compute_tokens<T>(chat: &T)
    where
        T: Token, // By using generics, we create typed functions at compile time
    {
        println!("[Generic mode]: {}", chat.compute_tokens());
    }
    generic_compute_tokens(&new_chat);

    // Generics shorthand alternative 1
    fn generic_compute_tokens_shorthand<T: Token>(chat: &T) {
        println!("[Generic shorthand] :{}", chat.compute_tokens());
    }

    // Generics shorthand alternative 2
    fn generic_compute_tokens_shorthand_2(chat: &impl Token) {
        println!("[Generic shorthand 2]: {}", chat.compute_tokens());
    }
    generic_compute_tokens(&new_chat);
    generic_compute_tokens_shorthand(&new_chat);
    generic_compute_tokens_shorthand_2(&new_chat);

    /*
     * In Rust, a Box is a smart pointer that allocates data on the heap.
     * When you create a Box, it stores its data on the heap, and the Box pointer itself resides on the stack.
     * Box is a struct known as a smart pointer that JUST holds the pointer to our data on the heap.
     * Box is often used as a way to store a reference to something in a struct that must know the size of its fillds.
     */
    struct LLMEcosystem {
        types: Vec<Box<dyn Token>>, // ChatCompletionMessage is one of the LLM applications. There are plenty of others like image, audio and assistant
    }
    let total_tokens_by_user = LLMEcosystem {
        types: vec![
            Box::new(ChatCompletionMessage {
                role: "devloper".to_string(),
                content: "Golang vs. Rust".to_string(),
                last_response: None,
            }),
            Box::new(ChatCompletionMessage {
                role: "devloper".to_string(),
                content: "Rust Learning Map".to_string(),
                last_response: None,
            }),
        ],
    };
    for (idx, data) in total_tokens_by_user.types.iter().enumerate() {
        println!("[Box pointer]{}: {}", idx, data.compute_tokens());
    }
}

fn smart_pointers() {
    /*
     * Reference can be converted into a more primitive type called a raw pointer.
     * *const T - A raw pointer to data of type T that should never change.
     * *mut T - A raw pointer to data of type T that can change.
     *
     * Raw pointers can be converted to and from numbers (e.g. usize).
     */
    let foo = 42;
    let memory_location = &foo as *const i32 as usize;
    println!("Memory location of foo: {}", memory_location);

    /*
     * In addition to the ability to create references to existing typed data using the & operator,
     * Rust gives us the ability to create reference-like structs called smart pointers.
     *
     * Typically smart pointers implement Deref, DerefMut, and Drop traits to
     * specify the logic of what should happen when the structure is dereferenced with * and . operators.
     */
    struct TattleTell<T> {
        value: T,
    }
    impl<T> Deref for TattleTell<T> {
        type Target = T; // Represents get T after dereferencing
        fn deref(&self) -> &T {
            println!("{} was used!", std::any::type_name::<T>());
            &self.value
        }
    }
    let foo = TattleTell {
        value: "secret message",
    };
    println!("{}", foo.len()); // TattleTell doesn't define a len() method. Rust attempts to deref automatically.

    /*
     * Use unsafe when absolutely necessary and only after exhausting all safe alternatives.
     * The Rust philosophy encourages minimal use of unsafe, ensuring memory safety and concurrency guarantees.
     */
    let a: [u8; 4] = [0, 1, 2, 3];
    let pointer_a = &a as *const u8 as usize;
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);

    /*
     * The standard library has a universal trait std::error::Error for describing errors.
     *
     * Using a smart pointer Box we can use the type Box<dyn std::error::Error> as a common type for returning errors
     * because it allows us to propagate up an error on the heap and interact with it at a high level without having to know a specific type.
     */
    struct Pie;

    #[derive(Debug)]
    struct NotFreshError;

    impl Display for NotFreshError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "This pie is not fresh")
        }
    }

    impl Error for NotFreshError {}

    impl Pie {
        fn eat(&self) -> Result<(), Box<dyn Error>> {
            Err(Box::new(NotFreshError))
        }
    }

    // Return a type capable of describing almost any kind of error that might occur in our program
    // so long as the error's data structure implements Rust's common Error trait
    fn eat_pie() -> Result<(), Box<dyn Error>> {
        let heap_pie = Box::new(Pie);
        heap_pie.eat()?; // The ? operator propagates the error if one occurs. It expands to an early return Err(From::from(err))
        Ok(())
    }

    match eat_pie() {
        Ok(()) => println!("Yummy!"),
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn tokio_async_programming() {
    /*
     * Tokio is able to concurrently run many tasks on a few threads by repeatedly swapping the currently running task on each thread
     * However, this kind of swapping can only happen at .await points, so code that spends a long time without reaching an .await
     * will prevent other tasks from running.
     * To combat this, Tokio provides two kinds of threads: Core threads and blocking threads.
     *
     * The core threads are where all asynchronous code runs, and Tokio will by default spawn one for each CPU core.
     * You can use the environment variable TOKIO_WORKER_THREADS to override the default value.
     *
     * The blocking threads are spawned on demand, can be used to run blocking code that would otherwise block other tasks from running
     * and are kept alive when not used for a certain amount of time which can be configured with thread_keep_alive.
     */

    // This is running on a core thread.
    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread. Blocking here is ok.
        // Simulate a blocking operation with thread::sleep
        thread::sleep(Duration::from_secs(1));
        println!("Blocking operation: 1000 ms have elapsed");

        42
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the panic.
    blocking_task.await.unwrap();

    // An fn for simulating a CPU intensive task
    fn fibonacci(n: i8) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    let n: i8 = 40;
    let blocking_task2 = tokio::task::spawn_blocking(move || {
        let result = fibonacci(n);
        println!("Fibonacci({}) = {}", n, result);
        result
    });

    blocking_task2.await.unwrap();
}

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("\n--Use `{{}}` for arguments--");
    println!();
    println!("\tprintln!(\"{{}} days\", 31);");
    print!("\n\tOutput: ");
    println!("{} days", 31);
    println!();

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("--Positional arguments--");
    println!();
    println!("\tprintln!(\"{{0}}, this is {{1}}. {{1}}, this is {{0}}\", \"Alice\", \"Bob\");");
    print!("\n\tOutput: ");
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!();

    // As can named arguments.
    println!("--Named arguments--");
    println!();
    println!("\tprintln!(\" {{subject}} {{verb}} {{object}}\",
             object=\"the lazy dog\",
             subject=\"the quick brown fox\",
             verb=\"jumps over\");");
             print!("\n\tOutput: ");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!();

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("--Formatting can be invoked by specifying the format character--");
    println!();
    
    println!("\t1");
    println!("\tprintln!(\"Base 10:               {{}}\",   69420); // 69420");
    print!("\n\tOutput: ");
    println!("Base 10:               {}",   69420); // 69420
    println!();

    println!("\t2");
    println!("\tprintln!(\"Base 2 (binary):       {{:b}}\", 69420); // 10000111100101100");
    print!("\n\tOutput: ");
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!();
    
    println!("\t3");
    println!("\tprintln!(\"Base 8 (octal):        {{:o}}\", 69420); // 207454\");");
    print!("\n\tOutput: ");
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!();

    println!("\t4");
    println!("\tprintln!(\"Base 16 (hexadecimal): {{:x}}\", 69420); // 10f2c");
    print!("\n\tOutput: ");
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!();

    println!("\t5");
    println!("\tprintln!(\"Base 16 (hexadecimal): {{:X}}\", 69420); // 10F2C");
    print!("\n\tOutput: ");
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C
    println!();

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("--Right-justify text with a specified width--");
    println!();
    println!("\tprintln!(\"{{number:>5}}\", number=1);");
    print!("\n\tOutput: ");
    println!("{number:>5}", number=1);
    println!();

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("--Pad numbers with extra zeroes--");
    println!();
    println!("\tprintln!(\"{{number:0<5}}\", number=1);");
    print!("\n\tOutput: ");
    println!("{number:0<5}", number=1);
    println!();

    // You can use named arguments in the format specifier by appending a `$`.
    println!("--Use named arguments in the format specifier by appending a `$`--");
    println!();
    println!("\tprintln!(\"{{number:0>width$}}\", number=1, width=5);");
    print!("\n\tOutput: ");
    println!("{number:0>width$}", number=1, width=5);
    println!();

    // Rust even checks to make sure the correct number of arguments are used.
    println!("--Correctness of #arguments--");
    println!();
    println!("\tRust even checks to make sure the correct number of arguments are used. Thus below code will 
throw !!ERROR!!.
\n\tprintln!(\"My name is {{0}}, {{1}} {{0}}\", \"Bond\");");
    println!();
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("--For Rust 1.58 and above, you can directly capture the argument from a surrounding variable--");
    println!();
    println!("\tprintln!(\"{{number:>width$}}\");");
    print!("\n\tOutput: ");
    println!("{number:>width$}");

    println!("--Using formatting and precision--");
    println!();
    println!("\tPrint \"Pi is roughly 3.142 \"");
    let pi = 3.141592;
    println!("\tOutput: Pi is roughly {0:.3}",pi);

    println!();
    println!("NOTE : \n");
    println!("\tOnly types that implement fmt::Display can be formatted with `{{}}`. User-defined types do not implement fmt::Display by default.");
    println!();
}
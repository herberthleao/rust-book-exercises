use std::io;

fn handle_menu() {
    clear();

    println!("1. Convert to Celsius");
    println!("2. Generate the nth Fibonacci number");
    println!("3. Print lyrics to the Christmas");
    println!("0. Exit");

    let choice = read_input();
    let number: u32 = choice.parse().expect("Invalid choice");

    clear();
    match number {
        1 => convert_to_celsius(),
        2 => generate_fibonacci(),
        3 => print_christmas_lyrics(),
        _ => exit(),
    }

    exit();
}

fn read_input() -> String{
    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Invalid input.");

    value.trim().to_string()
}

fn clear() {
    println!("\x1Bc");
}

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}

fn convert_to_celsius() {
    println!("Insert a Fahrenheit value:");
    let input = read_input();

    let fahrenheit: f64 = input.parse().expect("Invalid Fahrenheit value");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    clear();
    println!("{} ºF is {} ºC", fahrenheit, celsius)
}

fn generate_fibonacci() {
    println!("Insert the number:");
    let input = read_input();
    clear();

    let number: u32 = input.parse().unwrap_or(0);
    let mut a = 0;
    let mut b = 1;
    let mut result = a;

    if number > 0 {
        if number > 1 {
            result = b;
        }
        if number > 2 {
            for _ in 0..=number - 2 {
                let temp = b;
                b = a + b;
                a = temp;

                result = temp;
            }
        }

        println!("{}", result);
    }
}

fn print_christmas_lyrics() {
    let gifts = [
        ["first", "A partridge in a pear tree"],
        ["second", "Two turtle doves"],
        ["third", "Three French hens"],
        ["fourth", "Four calling birds"],
        ["fifth", "Five gold rings"],
        ["sixth", "Six geese a-laying"],
        ["seventh", "Seven swans a-swimming"],
        ["eighth", "Eight maids a-milking"],
        ["ninth", "Nine ladies dancing"],
        ["tenth", "Ten lords a-leaping"],
        ["eleventh", "Eleven pipers piping"],
        ["twelfth", "Twelve drummers drumming"],
    ];

    let mut counter = 0;
    for gift in gifts {
        println!("On the {} day of Christmas my true love sent to me", gift[0]);
        for n in (0..=counter).rev() {
            println!("{}", gifts[n][1]);
        }
        println!();
        counter += 1;
    }
}

fn main() {
    loop {
        handle_menu();
    }
}

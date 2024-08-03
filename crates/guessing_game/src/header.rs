const AUTHOR: &str = "Herberth Leão";
const EMAIL: &str = "dev@herberthleao.com";
const VERSION: &str = "v0.1.0";

/// Draws the full header.
/// 
/// # Arguments
/// 
/// * `title` - The program title.
pub fn draw_header(title: &str) {
    clear();

    write_title(title, VERSION);
    write_signature(AUTHOR, EMAIL);

    println!();
}

/// Writes the program title.
/// 
/// # Arguments
/// 
/// * `title` - The program title.
/// * `version` - The program version.
fn write_title(title: &str, version: &str) {
    draw_horizontal_border();
    draw_vertical_left_border(13);
    print!("{title} - {version}");
    draw_vertical_right_border(13);
}

/// Writes the signature.
/// 
/// # Arguments
/// 
/// * `author` - The program author name.
/// * `email` - The program author email.
fn write_signature(author: &str, email: &str) {
    draw_horizontal_border();

    draw_vertical_left_border(1);
    print!("Author: {author} <{email}>");
    draw_vertical_right_border(6);

    draw_horizontal_border();
}

/// Clear the terminal.
fn clear() {
    println!("\x1Bc");
}

/// Draws a horizontal border.
fn draw_horizontal_border() {
    for _ in 0..53 {
        print!("*");
    }
    println!();
}

/// Draws a vertical border on the left.
/// 
/// Draws a vertical border on the left for the current line, with padding on
/// the right.
///
/// # Arguments
/// 
/// * `padding` - The right padding size.
fn draw_vertical_left_border(padding: u32) {
    print!("*");

    for _ in 0..padding {
        print!(" ");
    }
}

/// Draws a vertical border on the right.
/// 
/// Draws a vertical border on the right for the current line, with padding on
/// the left.
///
/// # Arguments
/// 
/// * `padding` - The left padding size.
fn draw_vertical_right_border(padding: u32) {
    for _ in 0..padding {
        print!(" ");
    }

    println!("*");
}
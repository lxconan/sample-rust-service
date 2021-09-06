use colored::Colorize;

pub fn print_error(message: &str) {
    println!("{}: {}", "error".bright_red(), message);
}
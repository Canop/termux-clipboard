//! This example just prints the content of the android clipboard

fn main() {
    println!("Clipboard Content: {:?}", termux_clipboard::get_string());
}

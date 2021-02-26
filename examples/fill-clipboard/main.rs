//! This example writes a simple string into the android clipboard

fn main() {
    let con = "This clipboard content was written by the termux_clipboard crate";
    termux_clipboard::set_string(con).unwrap();
    println!("done, I've filled your clipboard");
}

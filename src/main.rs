use arboard::Clipboard;
use std::env;

fn main() {
  let arg = env::args().nth(1);

  match arg {
    Some(text) => {
      let mut clipboard = Clipboard::new().unwrap();
      clipboard.set_text(text).unwrap();
    }
    None => {
      eprintln!("Error: provide an argument")
    }
  }
}

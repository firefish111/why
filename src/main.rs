mod why;
use why::Why;

fn main() {
  let mut yes = Why::new();
  let mut no = Why::new();
  yes += 14;
  no += 9;

  yes += no;
  yes += -4;
  println!("{}", -yes);
}

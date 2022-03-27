mod why;
use why::Why;

fn main() {
  let yes = Why::new();
  let no = Why::new();
  let yes = yes + 14;
  let no = no + 9;

  let yes = yes + no;
  let yes = yes - 4;
  println!("Hello, world! {}", yes);
}

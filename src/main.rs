mod why;
use why::Why;

fn main() {
  let yes = Why::new();
  let yes = (yes + 14) + 9;
  println!("Hello, world! {:?}", yes);
}

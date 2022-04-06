extern crate reqwest; 

mod why;
use why::Why;

fn send(input: Why) -> Result<(), Box<dyn std::error::Error>> {
  println!("doing res");
  let client = reqwest::blocking::Client::new();
  let res = client.post("http://192.168.0.5:7480/post")
    .json(&input)
    .send()?;

  println!("done {:#?}", res);

  Ok(())
}

fn main() {
  let mut yes = Why::new(0);
  let mut no = Why::new(0);
/*  yes += 14;
  no += 9;

  yes += no;
  yes -= 4;
  yes = -yes;*/
  no += 99;
  yes -= 67;
  yes = yes * no;

  println!("{}", yes);
  send(yes); 
}

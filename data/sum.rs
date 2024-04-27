use std::io::Write;

#[allow(dead_code)]
fn input() -> f64 {
     let mut text = String::new();
     eprint!("? ");
     std::io::stderr().flush().unwrap();
     std::io::stdin()
         .read_line(&mut text)
         .expect("Cannot read line.");
     text.trim().parse::<f64>().unwrap_or(0.)
}

fn main() {
   let mut _a = 0.0;
   let mut _b = 0.0;
   let mut _c = 0.0;
   _a = input();
   _b = input();
   _c = 6f64;
   println!("{}", a + b / b + c);
}

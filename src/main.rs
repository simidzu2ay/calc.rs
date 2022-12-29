use lib::calc;

mod lib;

fn main() {
    match calc("2 +2 ") {
        Ok(v) => println!("{}", v),
        Err(err) => eprintln!("{:?}", err),
    }
}

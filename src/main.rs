#[macro_use]
use name_macro::*;

#[derive(Values, Debug)]
enum BadLanguages {
    Python
}



fn main() {
    println!("{:?}", BadLanguages::values())
}

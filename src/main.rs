use std::fmt;
use std::cmp;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

trait Asdf {
    fn to_lowercase(&self) -> &str;
}

impl Asdf for str {
    fn to_lowercase(&self) -> &str {
        println!("pog");
        "pog"
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> { 
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", cmp::min(x+1,y+1)).expect("Failed to write!");
            }
            if y != self.height-1 {
                write!(f, "{}", "\n").expect("Failed to write!");
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    <str as Asdf>::to_lowercase("A");
    str::to_lowercase("A");
    "A".to_lowercase();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 4, height: 4 },
    ];

    list.sort_by_key(|r| r.width);
    for rectangle in list {
        println!("{:#?}", rectangle)
    }
}

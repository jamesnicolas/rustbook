use std::fmt;
use std::cmp;

struct Rectangle {
    width: u32,
    height: u32,
}

trait Asdf {
    fn pog(&self);
}

impl Asdf for str {
    fn pog(&self) {
        println!("pog")
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> { 
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", cmp::min(x+1,y+1));
            }
            if y != self.height-1 {
                write!(f, "{}", "\n");
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    "a".pog();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 4, height: 4 },
    ];

    list.sort_by_key(|r| r.width);
    for rectangle in list {
        println!("{}", rectangle)
    }
}

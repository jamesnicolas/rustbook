struct Grid([[i32;32]; 32]);

impl Grid {
    fn print(&self) {
        for i in self.0 {
            for j in i {
                print!("{}",j)
            }
            println!("")
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut grid = Grid([[0;32]; 32]);

    grid.0[0][0] = 3;

    grid.print();
}

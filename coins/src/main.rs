#[repr(isize)]
enum Coin {
    Penny = 1,
    Nickel = 5,
    Dime = 10,
    Quarter = 25,
    Loonie = 100,
    Toonie = 200,
}

fn main() {
    let mut wallet: Vec<Coin> = vec![];

    wallet.push(Coin::Penny);
    wallet.push(Coin::Penny);
    wallet.push(Coin::Penny);
    wallet.push(Coin::Penny);
    wallet.push(Coin::Dime);
    wallet.push(Coin::Dime);
    wallet.push(Coin::Dime);
    wallet.push(Coin::Dime);
    wallet.push(Coin::Nickel);
    wallet.push(Coin::Nickel);
    wallet.push(Coin::Nickel);
    wallet.push(Coin::Nickel);
    wallet.push(Coin::Quarter);
    wallet.push(Coin::Quarter);
    wallet.push(Coin::Quarter);
    wallet.push(Coin::Quarter);
    wallet.push(Coin::Quarter);
    wallet.push(Coin::Loonie);
    wallet.push(Coin::Toonie);

    let mut sum: isize = 0;

    for coin in wallet {
        sum += coin as isize
    }

    println!("I have ${}.{}", sum/100, sum % 100);
}

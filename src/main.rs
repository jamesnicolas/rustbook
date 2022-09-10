fn main() {
    let result;
    let string1 = "abcd";
    {
        let string2 = "abcdefg";
        result = longest(string1, string2);
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
fn consume_with_relish<F>(func: F) where F: FnOnce() -> String {
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}

fn main() {
    let x = String::from("x");
    //let x = 5;
    let consume_and_return_x = || format!("{}", x);
    consume_with_relish(consume_and_return_x);
    consume_and_return_x();
    println!("{}", x);
}
*/

fn main() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    assert_eq!(maybe_some_len, Some(13));
    println!("{:?}", maybe_some_string);
}
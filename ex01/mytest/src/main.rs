fn main() {
    let mut s = String::from("test");

    let r3 = &mut s;
    r3.push_str("AAA");
    
    

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    println!("{}", r3);
}

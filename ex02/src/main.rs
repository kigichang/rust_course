fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test() {
    ()
}

fn main() {
    let mut s = String::from("hello world");

    let word:&str = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {}", word);


    let tmp: Vec<i32> = vec![1, 2, 3];

    let x3:i32 = tmp[2];
    let ref rx3 = tmp[2];
    let rx33: &i32 = &tmp[2];

    let _xx: &[i32] = &tmp;

    println!("{}", x3);
    println!("{}", rx3);
    println!("{}", rx3 == rx33);

    let my_str = "aaaa";
    let my_string = String::from(my_str);

    format!("{}", "A")
}
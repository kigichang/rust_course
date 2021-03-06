/*fn main() {
    let mut s = String::from("test abava");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    r3.push_str("AAA");

    //println!("{} {}", r1, r2);

    println!("{}", r3);


    let s1: &str = &s[0..3];

    println!("{}", s1);
}
*/
fn main() {
    println!("Hello, world!");

    let r = &4;
    println!("{:?}", r);

    match r {
        //&val => println!("& {:?}", val),
        //ref val => println!("ref {:?}", val),
        x => println!("any {:?}", x),
    }

    let mut v = 5;

    match v {
        ref r => println!("get ref {:?}", r),
    }

    match v {
        ref mut r => {
            *r += 10;
        },
    }

    println!("{:?}", v);

    match v {
        mut mutv => {
            mutv += 10;
        },
    }

    println!("{:?}", v);
}
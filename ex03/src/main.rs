fn main() {
    println!("Hello, world!");

    let r = &4;
    println!("{:?}", r);

    match r {
        //&val => println!("& {:?}", val),
        ref val => println!("ref {:?}", val),
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

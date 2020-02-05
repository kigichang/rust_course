/*
use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn test_point(p: &Point) {
    println!("{:?}", p);
}

// 使用 Drop ，就不用能 Copy
impl Drop for Point {
    fn drop(&mut self) {
        println!("{:#?} dropped", self);
    }
}

fn main() {
    let mut box_point = Box::new(Point{x: 0.0_f64, y:0.0_f64});
    test_point(&box_point);

    box_point.x = 100.0_f64;
    test_point(&box_point);

    let point = Point{x: 1.0_f64, y: 1.0_f64};

    println!("{}", mem::size_of_val(&point));

    println!("{}", mem::size_of_val(&box_point));
    println!("{}", mem::size_of_val(&(*box_point)));
}
*/
/*
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn eat(mut p: Point) {
    p.y = 200.0_f64;
    println!("{:?}", p);
}


fn main() {
    let mut p = Point {x: 0.0_f64, y: 0.0_f64};

    eat(p);
    p.x = 10.0_f64;
    println!("{:?}", p);
}
*/
/*
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
*/
struct Inner;
struct Outer(Inner);

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Dropping Inner!");
    }
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("Dropping Outer!");
    }
}

fn main() {
    let x = Outer(Inner);
    let _y = Inner;
    let z = 10;

    // x.drop(); // explicit destructor calls not allowed

    std::mem::drop(x);
    std::mem::drop(z);

    println!("end");
}
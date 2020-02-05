use std::iter::Map;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for _ in v.iter().map(|x| println!("{}", x)) {
        
    }
}

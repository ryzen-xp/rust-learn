// Topic: map
// Run with:  cargo run --bin map

fn main() {
    let v: Vec<i32> = vec![1, 3, 4, 5];

    let double: Vec<i32> = v.iter().map(|x| x * 2).collect();

    let x: Vec<&i32> = v.iter().collect();

    println!("{:?}", double);
}

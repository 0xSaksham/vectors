fn main() {
    let v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    println!("The value at Second index is {third}");

    let fourth: Option<&i32>= v.get(3);
    match fourth{
        Some(fourth) => println!("The element at third index is {fourth}"),
        None => println!("There is no element at third index")
    }
}
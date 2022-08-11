fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v4 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v4[100]; // panic
    let does_not_exist = v4.get(100); // None

    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];

    // cannot borrow `v5` as mutable because it is also borrowed as immutable mutable borrow occurs
    // v5.push(6);

    println!("The first element is: {}", first);
}

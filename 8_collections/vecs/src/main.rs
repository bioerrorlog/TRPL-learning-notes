fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    ////////////////////////////////////////////////////////////
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    ////////////////////////////////////////////////////////////
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // None

    ////////////////////////////////////////////////////////////
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // cannot borrow `v` as mutable because it is also borrowed as immutable mutable borrow occurs
    // v.push(6);

    println!("The first element is: {}", first);

    ////////////////////////////////////////////////////////////
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

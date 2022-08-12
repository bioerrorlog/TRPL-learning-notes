fn main() {
    // ----------------------------------------
    let mut s = String::new();

    // ----------------------------------------
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contets".to_string();

    // ----------------------------------------
    let s = String::from("initial contents");
}

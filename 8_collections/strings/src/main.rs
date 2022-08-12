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

    // ----------------------------------------
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // ----------------------------------------
    let mut s = String::from("foo");
    s.push_str("bar");

    // ----------------------------------------
    let mut s1 = String::from("foo");
    // let s2 = String::from("bar");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // ----------------------------------------
    let mut s = String::from("lo");
    s.push('l');
}

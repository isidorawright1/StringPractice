fn main() {
    //greeting("Hello Message");
    //let message = "hello world";
    //let mut message = String::from("hello world");
    //greeting(&mut message);
    //greeting(&message);
    //println!("{}", message);

    let mut message = String::from("dog");
    greeting(&mut message);
}

fn greeting(message: &mut String) {
    //message.push_str(" adding data");
    *message = "cat".to_string();
    println!("{}", message);
}
#[allow(dead_code)] // Added to avoid warning of unused in compilation
struct User {
    name: String,
    age: u32,
    gender: String,
    active: bool,
}

fn main() {
    let user1 = User {
        name: String::from("demo"),
        age: 20,
        active: true,
        gender: String::from("male"),
    };
    println!("User with name {:?} , of age {:?}", user1.name, user1.age);
}
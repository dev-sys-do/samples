#[derive(Debug)]
struct Student<'a, 'b> {
    age: u8,
    name: &'a str,
    nick: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn new(age: u8, name: &'a str, nick: &'b str) -> Self {
        Student { age, name, nick }
    }
}

fn main() {
    let name = String::from("Robert");
    let nick = String::from("Bob");

    println!("New student: {:?}", Student::new(25, &name, &nick));
}

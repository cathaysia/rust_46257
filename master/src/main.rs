use slave::print_hello;

#[derive(Debug)]
struct Age {
    age: u8,
}

fn main() {
    let age = Age {
        age: Default::default(),
    };
    if age.age == Default::default() {
        print_hello()
    }
}

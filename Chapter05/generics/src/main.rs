#[derive(Debug)]
struct Data<T> {
    value: T,
}

impl<T> Data<T> {
    fn print_me(&self) {
        println!("Hello its me!");
    }
}

fn main() {
    let char = Data { value: 'a' };
    let int = Data { value: 96 };
    let float = Data { value: 96.0 };

    char.print_me();
    int.print_me();
    float.print_me();
}

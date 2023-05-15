#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin,
}

impl PermissionLevel {
    fn description(&self) -> String {
        match self {
            PermissionLevel::User => String::from("I am an User"),
            PermissionLevel::Instructor => String::from("I am an Instructor"),
            PermissionLevel::Admin => String::from("I am an Admin"),
        }
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("{}", user1.description());

    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.description());

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.description());
}

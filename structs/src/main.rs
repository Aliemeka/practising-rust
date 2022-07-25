fn main() {
    let dome = Dao {
        name: String::from("Dome Academy"),
        description: String::from("Platform of shared learning"),
        no_of_members: 90,
    };
    dome.print_name();
    dome.print_description();
    dome.print_no_of_members();
}

struct Dao {
    name: String,
    description: String,
    no_of_members: u128,
}

impl Dao {
    fn print_name(&self) {
        println!("{}", self.name);
    }
    fn print_description(&self) {
        println!("{}", self.description);
    }
    fn print_no_of_members(&self) {
        println!("Dao has {} members", self.no_of_members);
    }
}

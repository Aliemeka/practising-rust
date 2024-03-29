fn main() {
    let name = "Dome Academy";
    let dome = Dao::new(name, "Platform of shared learning", 90);
    dome.print_name();
    dome.print_description();
    dome.print_no_of_members();
    println!("{}", dome.can_build());
    println!("{}", dome.is_growing());
}

struct Dao {
    name: String,
    description: String,
    no_of_members: u128,
}

impl Dao {
    fn new(name: &str, description: &str, no_of_members: u128) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            no_of_members,
        }
    }
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

impl Group for Dao {
    fn can_build(&self) -> bool {
        true
    }
}

trait Group {
    fn can_build(&self) -> bool;
    fn is_growing(&self) -> bool {
        true
    }
}

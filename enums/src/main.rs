fn main() {
    let a: Level = Level::Primary;
    let b: Level = Level::Secondary(8);
    let c: Level = Level::Tertiary { x: 10, y: 30 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let Level::Secondary(val) = b {
        println!("{}", val);
    }
    if let Level::Tertiary { x, y } = c {
        println!("{},{}", x, y)
    }
}

#[derive(Debug)]
enum Level {
    Primary,
    Secondary(i32),
    Tertiary { x: i32, y: i32 },
}

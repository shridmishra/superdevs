struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
}
impl Shape for Square {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

fn main() {
    let r = Rect {
        width: 30,
        height: 20,
    };
    let s = Square { side: 30 };

    print!("{}", r.area());
    print!("{}", s.area());
}

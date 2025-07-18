struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}
impl Shape for Square {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }

    fn perimeter(&self) -> u32 {
        return self.side * 4;
    }
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
    fn perimeter(&self) -> u32 {
        return (self.height + self.width) * 2;
    }
}

fn main() {
    let r = Rect {
        width: 30,
        height: 20,
    };
    let s = Square { side: 30 };
    let (area,perimeter) = get_area_and_perimeter(s);  
}

fn get_area_and_perimeter(s: impl Shape) -> (u32, u32) {
    return (s.area(), s.perimeter());
}

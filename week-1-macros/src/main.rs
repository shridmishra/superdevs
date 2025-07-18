 
// macro_rules! vector {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

// fn main() {
// 	let ans = vector!(1, 2, 3);
// 	println!("{}", ans[0]);
// }



trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: Vec<u8>) -> Result<Swap, std::fmt::Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Deserialize for Swap {
    fn deserialize(data: Vec<u8>) -> Result<Swap, std::fmt::Error> {
        if data.len() < 8 {
            return Err(std::fmt::Error);
        }

        let qty_1 = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

        Ok(Swap { qty_1, qty_2 })
    }
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        v
    }
}

fn main() {
    let v = Swap { qty_1: 1, qty_2: 2 };

    let ser = v.serialize();
    println!("{:?}", ser);

    let des = Swap::deserialize(ser).unwrap();
    println!("{:?}", des);
}
 
use std::mem::swap;
use std::ops::Add;

pub struct Triangle {
    is_equailateral_res: bool, 
    is_scalene_res: bool, 
    is_isosceles_res: bool, 
}

impl Triangle {
    pub fn build<T: PartialEq + Copy + PartialOrd + Add<Output = T>>(sides: [T; 3]) -> Option<Triangle> {
        let mut x = sides[0];
        let mut y = sides[1];
        let mut z = sides[2];
        if x > y {
            swap(&mut x, &mut y);
        }
        if x > z {
            swap(&mut x, &mut z);
        }

        if y > z {
            swap(&mut y, &mut z);
        }

        if z >= x + y {
            return None;
        }
        else {
            return Some(Self{
                is_equailateral_res: if x == z {true} else {false},
                is_scalene_res: if x != y && y != z {true} else {false},
                is_isosceles_res: if x == y || y == z {true} else {false},
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.is_equailateral_res
    }

    pub fn is_scalene(&self) -> bool {
        self.is_scalene_res
    }

    pub fn is_isosceles(&self) -> bool {
        self.is_isosceles_res
    }
}

// impl Triangle {
//     pub fn build(sides: [u64; 3]) -> Option<Triangle> {
//         let mut x = sides[0];
//         let mut y = sides[1];
//         let mut z = sides[2];
//         if x > y {
//             swap(&mut x, &mut y);
//         }
//         if x > z {
//             swap(&mut x, &mut z);
//         }

//         if y > z {
//             swap(&mut y, &mut z);
//         }

//         if z >= x + y {
//             return None;
//         }
//         else {
//             return Some(Self{
//                 is_equailateral_res: if x == z {true} else {false},
//                 is_scalene_res: if x != y && y != z {true} else {false},
//                 is_isosceles_res: if x == y || y == z {true} else {false},
//             })
//         }
//     }

//     pub fn is_equilateral(&self) -> bool {
//         self.is_equailateral_res
//     }

//     pub fn is_scalene(&self) -> bool {
//         self.is_scalene_res
//     }

//     pub fn is_isosceles(&self) -> bool {
//         self.is_isosceles_res
//     }
// }

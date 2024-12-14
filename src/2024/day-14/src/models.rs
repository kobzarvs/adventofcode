use geo::Coord;

#[cfg(not(debug_assertions))]
pub const WIDTH: i32 = 101;
#[cfg(debug_assertions)]
pub const WIDTH: i32 = 11;

#[cfg(not(debug_assertions))]
pub const HEIGHT: i32 = 103;
#[cfg(debug_assertions)]
pub const HEIGHT: i32 = 7;

#[cfg(not(debug_assertions))]
pub const CENTER: i32 = 50;
#[cfg(debug_assertions)]
pub const CENTER: i32 = 5;

#[cfg(not(debug_assertions))]
pub const MIDDLE: i32 = 51;
#[cfg(debug_assertions)]
pub const MIDDLE: i32 = 3;


#[derive(Debug, Clone)]
pub struct Robot {
    pub p: Coord<i32>,
    pub v: Coord<i32>,
}

impl Robot {
    pub fn do_move(&mut self) -> Coord<i32> {
        self.p.x = (self.p.x + self.v.x).rem_euclid(WIDTH);
        self.p.y = (self.p.y + self.v.y).rem_euclid(HEIGHT);
        self.p
    }
}
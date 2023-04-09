use std::cmp::min;

#[derive(Debug)]
pub struct RectangularPrism {
    pub height: u32,
    pub length: u32,
    pub width: u32,
}

impl RectangularPrism {
    pub fn area(&self) -> u32 {
        2 * (self.height * self.width + self.height * self.length + self.length * self.width)
    }
}

pub fn paper_needed(rp: &RectangularPrism) -> u32 {
    let smallest = min(rp.height, min(rp.length, rp.width));
    let second_smallest = if smallest == rp.height {
        min(rp.length, rp.width)
    } else if smallest == rp.length {
        min(rp.height, rp.width)
    } else {
        min(rp.height, rp.length)
    };

    rp.area() + smallest * second_smallest
}

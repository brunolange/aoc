#[derive(Debug)]
pub struct GiftBox {
    pub height: u32,
    pub length: u32,
    pub width: u32,
}

impl GiftBox {
    pub fn area(&self) -> u32 {
        2 * (self.height * self.width + self.height * self.length + self.length * self.width)
    }
}

pub fn paper_needed(b: &GiftBox) -> u32 {
    let mut ordered = [b.height, b.length, b.width];
    ordered.sort();

    let [smallest, second_smallest, ..] = ordered;

    b.area() + smallest * second_smallest
}

pub fn line_to_box(line: &str) -> GiftBox {
    let dimensions: Vec<u32> = line.split("x").map(|x| x.parse().unwrap()).collect();
    GiftBox {
        height: dimensions[0],
        length: dimensions[1],
        width: dimensions[2],
    }
}

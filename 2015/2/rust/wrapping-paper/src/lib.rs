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

pub fn line_to_box(line: &str) -> Result<GiftBox, String> {
    let parse = |s: &str| s.parse();
    let dimensions: Result<Vec<u32>, _> = line.split("x").map(parse).collect();
    match dimensions {
        Err(err) => Err(format!("Found invalid dimension: {}", err)),
        Ok(dimensions) => {
            if dimensions.len() != 3 {
                return Err(format!("Expected line to contain 3 dimensions"));
            }
            Ok(GiftBox {
                height: dimensions[0],
                length: dimensions[1],
                width: dimensions[2],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let line = "1x1x10";
        let b = line_to_box(line).unwrap();
        assert_eq!(b.area(), 42);
    }
}

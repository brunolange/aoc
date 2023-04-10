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

    pub fn volume(&self) -> u32 {
        return self.height * self.length * self.width
    }

    pub fn sorted_dimensions(&self) -> [u32; 3] {
        let mut ordered = [self.height, self.length, self.width];
        ordered.sort();

        ordered
    }
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

pub fn paper_needed(b: &GiftBox) -> u32 {
    let [smallest, second_smallest, ..] = b.sorted_dimensions();

    b.area() + smallest * second_smallest
}

pub fn ribbon_needed(b: &GiftBox) -> u32 {
    let [smallest, second_smallest, ..] = b.sorted_dimensions();
    return 2 * (smallest + second_smallest) + b.volume()
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

    #[test]
    fn volume() {
        let line = "2x3x4";
        let b = line_to_box(line).unwrap();
        assert_eq!(b.volume(), 24);
    }    

    #[test]
    fn wrapping_paper_area() {
        let line = "1x1x10";
        let b = line_to_box(line).unwrap();
        assert_eq!(paper_needed(&b), 43);
    }

    #[test]
    fn ribbon_length_1() {
        let line = "2x3x4";
        let b = line_to_box(line).unwrap();
        assert_eq!(ribbon_needed(&b), 34);
    }

    #[test]
    fn ribbon_length_2() {
        let line = "1x1x10";
        let b = line_to_box(line).unwrap();
        assert_eq!(ribbon_needed(&b), 14);
    }
}

use log::warn;
use std::collections::HashMap;

pub mod models; // made pub just because of GridPoint doctest
use models::{GridPoint, Op};

mod parsers;

pub fn total_brightness(lines: impl Iterator<Item = String>) -> usize {
    let mut brightness_map: HashMap<GridPoint, usize> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let op = line.parse::<Op>();
        match op {
            Ok(op) => execute(&mut brightness_map, op),
            Err(_) => {
                warn!("Ignoring line {}: [{}]", i + 1, line);
            }
        }
    }

    brightness_map.values().sum()
}

fn execute(brightness_map: &mut HashMap<GridPoint, usize>, op: Op) {
    match op {
        Op::Toggle(mut rect) => {
            for grid_point in rect.iter() {
                *brightness_map.entry(grid_point).or_insert(0) += 2;
            }
        }
        Op::Turn(true, mut rect) => {
            for grid_point in rect.iter() {
                *brightness_map.entry(grid_point).or_insert(0) += 1;
            }
        }
        Op::Turn(false, mut rect) => {
            for grid_point in rect.iter() {
                let _ = *brightness_map
                    .entry(grid_point)
                    .and_modify(|v| {
                        if *v == 0 {
                            return;
                        }
                        *v -= 1;
                    })
                    .or_insert(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::Rect;

    #[test]
    fn test_rect() {
        assert_eq!(
            Rect::new(&GridPoint { x: 10, y: 10 }, &GridPoint { x: 11, y: 11 }),
            Rect {
                bottom_left_corner: GridPoint { x: 10, y: 10 },
                top_right_corner: GridPoint { x: 11, y: 11 },
            }
        );
        assert_eq!(
            Rect::new(&GridPoint { x: 11, y: 11 }, &GridPoint { x: 10, y: 10 }),
            Rect {
                bottom_left_corner: GridPoint { x: 10, y: 10 },
                top_right_corner: GridPoint { x: 11, y: 11 },
            }
        );

        assert_eq!(
            Rect::new(&GridPoint { x: 10, y: 10 }, &GridPoint { x: 11, y: 9 }),
            Rect {
                bottom_left_corner: GridPoint { x: 10, y: 9 },
                top_right_corner: GridPoint { x: 11, y: 10 },
            }
        );
        assert_eq!(
            Rect::new(&GridPoint { x: 11, y: 9 }, &GridPoint { x: 10, y: 10 }),
            Rect {
                bottom_left_corner: GridPoint { x: 10, y: 9 },
                top_right_corner: GridPoint { x: 11, y: 10 },
            }
        );
    }

    #[test]
    fn test_toggle_parse() {
        assert_eq!(
            "toggle 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Toggle(Rect {
                bottom_left_corner: GridPoint { x: 1, y: 2 },
                top_right_corner: GridPoint { x: 3, y: 4 },
            })
        );

        assert!("toggle 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" toggle 1,2 through 3,4".parse::<Op>().is_err());
        assert!("toggle 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("toggle 1,2,3 through 4,5".parse::<Op>().is_err());
    }

    #[test]
    fn test_turn_on_parse() {
        assert_eq!(
            "turn on 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Turn(
                true,
                Rect {
                    bottom_left_corner: GridPoint { x: 1, y: 2 },
                    top_right_corner: GridPoint { x: 3, y: 4 },
                }
            )
        );

        assert!("turn on 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" turn on 1,2 through 3,4".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
    }

    #[test]
    fn test_turn_off_parse() {
        assert_eq!(
            "turn off 1,2 through 3,4".parse::<Op>().unwrap(),
            Op::Turn(
                false,
                Rect {
                    bottom_left_corner: GridPoint { x: 1, y: 2 },
                    top_right_corner: GridPoint { x: 3, y: 4 },
                }
            )
        );

        assert!("turn on 1,2 through 3,4 ".parse::<Op>().is_err());
        assert!(" turn on 1,2 through 3,4".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
        assert!("turn on 1,2,3 through 4,5".parse::<Op>().is_err());
    }
}

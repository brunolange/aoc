use log::warn;
use std::collections::HashMap;

pub mod models; // made pub just because of GridPoint doctest
use models::{GridPoint, Op};

mod parsers;

/// Sum all of brightnesses. Very informative description!
///
/// If a command can't be parsed, it is ignored.
///
/// # Example
/// ```rust
/// use lightgrid::total_brightness;
///
/// let commands = vec!["turn on 0,0 through 1,1".to_string()].into_iter();
/// assert_eq!(total_brightness(commands), 4);
///
/// let commands = vec![
///     "turn on 0,0 through 1,1",
///     "turn on 0,0 through 1,1",
/// ].into_iter().map(str::to_string);
/// assert_eq!(total_brightness(commands), 8);
///
/// let commands = vec![
///     "toggle 0,0 through 99,99",
/// ].into_iter().map(str::to_string);
/// assert_eq!(total_brightness(commands), 20000);
///
/// let commands = vec![
///     "toggle 0,0 through 99,99",
///     "turn off 0,0 through 99,99",
/// ].into_iter().map(str::to_string);
/// assert_eq!(total_brightness(commands), 10000);
///
/// let commands = vec![
///     "toggle 0,0 through 99,99",
///     "turn off 0,0 through 99,99",
///     "turn on 100,100 through 100,100",
/// ].into_iter().map(str::to_string);
/// assert_eq!(total_brightness(commands), 10001);
///
/// let commands = vec![
///     "some invalid command",
///     "turn on 0,0 through 0,0",
/// ].into_iter().map(str::to_string);
/// assert_eq!(total_brightness(commands), 1);
/// ```
pub fn total_brightness(commands: impl Iterator<Item = String>) -> usize {
    let mut brightness_map: HashMap<GridPoint, usize> = HashMap::new();

    for (i, line) in commands.enumerate() {
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

/// Carries out the operation on the light grid, mutating the brightness map as it goes.
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
                        *v -= if *v == 0 { 0 } else { 1 };
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

use itertools::Itertools;

fn main() {
    let volume: usize = 150;
    let containers: Vec<usize> = vec![
        11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3,
    ];

    let part1 = number_of_container_groups_that_add_to_volume(&containers, volume);
    println!("part 1 = {part1}");

    let part2 = number_of_smallest_container_groups_that_add_to_volume(&containers, volume);
    println!("part 2 = {part2}");
}

fn container_groups_that_add_to(
    containers: &Vec<usize>,
    volume: usize,
) -> impl Iterator<Item = Vec<usize>> {
    containers
        .clone()
        .into_iter()
        .powerset()
        .filter(move |cs| cs.iter().sum::<usize>() == volume)
}

fn number_of_container_groups_that_add_to_volume(containers: &Vec<usize>, volume: usize) -> usize {
    container_groups_that_add_to(containers, volume).count()
}

fn number_of_smallest_container_groups_that_add_to_volume(
    containers: &Vec<usize>,
    volume: usize,
) -> usize {
    let groups: Vec<Vec<usize>> = container_groups_that_add_to(containers, volume).collect();
    let smallest_size = groups
        .iter()
        .min_by_key(|group| group.len())
        .expect("found no groups")
        .len();

    groups
        .iter()
        .filter(|group| group.len() == smallest_size)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            number_of_container_groups_that_add_to_volume(&vec![20, 15, 10, 5, 5], 25),
            4
        );

        assert_eq!(
            number_of_smallest_container_groups_that_add_to_volume(&vec![20, 15, 10, 5, 5], 25),
            3
        );
    }
}

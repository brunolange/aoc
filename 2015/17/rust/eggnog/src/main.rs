use std::collections::{HashMap, HashSet};

fn main() {
    let volume: u32 = 25;
    let containers = vec![20, 15, 10, 5, 5];

    let answer = ways(volume, &containers);
    println!("Hello, containers: {:?} -> {}", containers, answer);
}

fn ways(volume: u32, containers: &[u32]) -> usize {
    fn _ways(
        volume: i32,
        containers: &[u32],
        answers: &mut Vec<Vec<u32>>,
        candidate: &mut Vec<u32>,
        depth: usize,
    ) {
        let indent = "  ".repeat(depth);
        println!(
            "{indent}checking volume = {volume} with containers {:?}",
            containers
        );
        if containers.len() == 0 || volume < 0 {
            println!("{indent}nope...");
            _ = candidate.pop();
        }
        if volume == 0 {
            // if seen.contains(candidate) {
            //     println!("{indent}already seen this one...");
            //     return 0;
            // } else {
            //     seen.insert(&candidate);
            //     println!("{indent}+1!");
            //     return 1;
            // }
            answers.push(candidate.to_vec());
            println!("{indent}+1!");
        }

        containers
            .iter()
            .enumerate()
            .for_each(|(index, container)| {
                println!("{indent}selected container {container}");
                let remaining: i32 = volume as i32 - *container as i32;
                let other_containers = if index < containers.len() {
                    [&containers[..index], &containers[(index + 1)..]].concat()
                } else {
                    (&containers[..index]).to_vec()
                };

                candidate.push(*container);
                _ways(
                    remaining,
                    // if remaining < 0 {
                    //     *container as u32
                    // } else {
                    //     remaining as u32
                    // },
                    &other_containers,
                    answers,
                    candidate,
                    depth + 1,
                );

                // if go {
                //     answers.push(container)
                // }
            });
        // println!("{indent}{:?}", output);
    }

    let mut answers: Vec<Vec<u32>> = vec![vec![]];
    let mut candidate: Vec<u32> = vec![];
    _ways(volume as i32, containers, &mut answers, &mut candidate, 0);

    for a in answers {
        println!("answer = {:?}", a);
    }

    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // assert_eq!(ways(100, &[0]), 0);
        assert_eq!(ways(1, &[1]), 1);
        assert_eq!(ways(2, &[1]), 1);
        assert_eq!(ways(100, &[1]), 1);
        assert_eq!(ways(2, &[1, 2]), 2);
        assert_eq!(ways(3, &[1, 2]), 3);
        assert_eq!(ways(10, &[2, 3]), 7);
        assert_eq!(ways(25, &[20, 15, 10, 5, 5]), 4);
    }
}

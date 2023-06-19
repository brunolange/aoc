use reindeer_olympics::{parse_line, race_1, race_2, Reindeer};

mod io;

fn main() {
    let reindeers = io::lines()
        .into_iter()
        .map(|line| {
            let (_, reindeer) = parse_line(&line).unwrap();
            reindeer
        })
        .collect::<Vec<Reindeer>>();

    let duration = io::duration();

    let (winner, distance) = race_1(&reindeers, duration).unwrap();
    println!(
        "{} is the winner of part 1 and has travelled {} kilometers.",
        winner.name, distance
    );

    let (winners, score) = race_2(&reindeers, duration);
    println!(
        "[{:?}] are the winners of part 2 with {} points.",
        winners, score
    );
}

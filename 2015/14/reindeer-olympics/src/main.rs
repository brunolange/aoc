use reindeer_olympics::{parse_line, race_1, race_2, Reindeer};

fn main() {
    let lines = vec![
        // "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        // "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        // "Slowy can fly 1 km/s for 1 seconds, but then must rest for 162 seconds.",
        // "SlowyII can fly 1 km/s for 2 seconds, but then must rest for 162 seconds.",
        "Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.",
        "Blitzen can fly 13 km/s for 4 seconds, but then must rest for 49 seconds.",
        "Rudolph can fly 20 km/s for 7 seconds, but then must rest for 132 seconds.",
        "Cupid can fly 12 km/s for 4 seconds, but then must rest for 43 seconds.",
        "Donner can fly 9 km/s for 5 seconds, but then must rest for 38 seconds.",
        "Dasher can fly 10 km/s for 4 seconds, but then must rest for 37 seconds.",
        "Comet can fly 3 km/s for 37 seconds, but then must rest for 76 seconds.",
        "Prancer can fly 9 km/s for 12 seconds, but then must rest for 97 seconds.",
        "Dancer can fly 37 km/s for 1 seconds, but then must rest for 36 seconds.",
    ];
    let reindeers = lines
        .into_iter()
        .map(|line| {
            let (_, reindeer) = parse_line(line).unwrap();
            reindeer
        })
        .collect::<Vec<Reindeer>>();

    let t = std::env::args()
        .nth(1)
        .unwrap_or("1000".to_owned())
        .parse::<usize>()
        .unwrap();

    let (winner, distance) = race_1(&reindeers, t).unwrap();

    println!(
        "{} is the winner of part 1 and has travelled {} kilometers.",
        winner.name, distance
    );

    let (winners, score) = race_2(&reindeers, t);

    println!(
        "[{:?}] are the winners of part 2 with {} points.",
        winners, score
    );
}

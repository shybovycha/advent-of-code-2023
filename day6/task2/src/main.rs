fn main() {
    // Time:      71530
    // Distance:  940200
    // let races = [(71530, 940200)];

    // Time:        45988373
    // Distance:   295173412781210
    let races = [(45988373, 295173412781210)];

    let res = races
        .iter()
        .map(|(d, r)| ways_to_go(*d, *r))
        .reduce(|acc, e| acc * e);

    println!("ways to win: {}", res.unwrap_or(0));
}

fn ways_to_go(duration: u64, previous_record: u64) -> u64 {
    (0..=duration)
        .map(|hold| hold * (duration - hold))
        .filter(|distance| distance > &previous_record)
        .count() as u64
}

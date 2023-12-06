fn main() {
    // Time:      7  15   30
    // Distance:  9  40  200
    // let races = [(7, 9), (15, 40), (30, 200)];

    // Time:        45     98     83     73
    // Distance:   295   1734   1278   1210
    let races = [(45, 295), (98, 1734), (83, 1278), (73, 1210)];

    let res = races
        .iter()
        .map(|(d, r)| ways_to_go(*d, *r))
        .reduce(|acc, e| acc * e);

    println!("ways to win: {}", res.unwrap_or(0));
}

fn ways_to_go(duration: u32, previous_record: u32) -> u32 {
    (0..=duration)
        .map(|hold| hold * (duration - hold))
        .filter(|distance| distance > &previous_record)
        .count() as u32
}

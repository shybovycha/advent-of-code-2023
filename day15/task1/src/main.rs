fn main() {
    // let s1 = include_str!("input1.txt");
    // let s2 = include_str!("input2.txt");

    let s_final = include_str!("input3.txt");

    let ans = solve(s_final);

    println!("ans = {}", ans);
}

fn hash(s: &str, start_value: u32) -> u32 {
    // let chunks = s.split(',')
    let mut value = start_value;

    for ch in s.chars() {
        if ch == '\n' {
            continue
        }

        value += ch as u32;
        value *= 17;
        value %= 256;
    }

    value
}

fn solve(s: &str) -> u32 {
    s.split(',').fold(0, |acc, x| acc + hash(x, 0))
}

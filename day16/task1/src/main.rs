use std::collections::{ VecDeque, HashSet };

fn main() {
    let s = include_str!("input.txt");

    let ans = solve(s.trim());

    println!("ans={}", ans);
}

fn solve(field: &str) -> usize {
    let lines = field.lines().collect::<Vec<_>>();
    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();

    par_solve(&lines, rows, cols, 0, 0, &Dir::Right)
}

fn par_solve(lines: &Vec<&str>, rows: usize, cols: usize, start_row: usize, start_col: usize, start_dir: &Dir) -> usize {
    let mut energized = HashSet::new();

    let mut q: VecDeque<(i32, i32, Dir)> = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((start_row as i32, start_col as i32, start_dir.clone()));

    while !q.is_empty() {
        let (row, col, dir) = q.pop_front().unwrap();

        let key = get_key(row, col, &dir);

        if visited.contains(&key) {
            continue
        }

        if row < 0 || row > (rows - 1) as i32 || col < 0 || col > (cols - 1) as i32 {
            continue
        }

        visited.insert(key);

        energized.insert((row, col));

        let ch = lines.get(row as usize).unwrap().chars().nth(col as usize).unwrap();

        if ch == '/' {
            if dir == Dir::Right { // && row > 0 {
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Left { // && row < rows - 1 {
                q.push_back((row + 1, col, Dir::Down));
            } else if dir == Dir::Up { // && col < cols - 1 {
                q.push_back((row, col + 1, Dir::Right));
            } else if dir == Dir::Down { // && col > 0 {
                q.push_back((row, col - 1, Dir::Left));
            }
        } else if ch == '\\' {
            if dir == Dir::Right { // && row > 0 {
                q.push_back((row + 1, col, Dir::Down));
            } else if dir == Dir::Left { // && row < rows - 1 {
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Up { // && col < cols - 1 {
                q.push_back((row, col - 1, Dir::Left));
            } else if dir == Dir::Down { // && col > 0 {
                q.push_back((row, col + 1, Dir::Right));
            }
        } else if ch == '-' {
            if dir == Dir::Right { // && row > 0 {
                q.push_back((row, col + 1, Dir::Right));
            } else if dir == Dir::Left { // && row < rows - 1 {
                q.push_back((row, col - 1, Dir::Left));
            } else if dir == Dir::Up { // && col < cols - 1 {
                q.push_back((row, col + 1, Dir::Right));
                q.push_back((row, col - 1, Dir::Left));
            } else if dir == Dir::Down { // && col > 0 {
                q.push_back((row, col + 1, Dir::Right));
                q.push_back((row, col - 1, Dir::Left));
            }
        } else if ch == '|' {
            if dir == Dir::Right {
                q.push_back((row + 1, col, Dir::Down));
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Left {
                q.push_back((row + 1, col, Dir::Down));
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Up {
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Down {
                q.push_back((row + 1, col, Dir::Down));
            }
        } else if ch == '.' {
            if dir == Dir::Right {
                q.push_back((row, col + 1, Dir::Right));
            } else if dir == Dir::Left {
                q.push_back((row, col - 1, Dir::Left));
            } else if dir == Dir::Up {
                q.push_back((row - 1, col, Dir::Up));
            } else if dir == Dir::Down {
                q.push_back((row + 1, col, Dir::Down));
            }
        }
    }

    energized.len()
}

fn get_key(row: i32, col: i32, dir: &Dir) -> String {
    std::format!("{},{},{}", row, col, dir_to_str(dir))
}

fn dir_to_str(dir: &Dir) -> &'static str {
    match dir {
        Dir::Right => "right",
        Dir::Left => "left",
        Dir::Up => "up",
        Dir::Down => "down",
    }
}

#[derive(PartialEq, Hash, Clone)]
enum Dir {
    Right,
    Up,
    Down,
    Left
}

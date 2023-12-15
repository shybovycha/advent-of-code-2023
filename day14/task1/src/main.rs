use std::collections::HashMap;

fn main() {
    let s1 = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    let s2 = r"
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
";

    let s_final = r".O.O.#........#..#...#..#....#...........#O.#.##.O#....#.......O......#..OO.#.#O....#O..O#...OO.#O..
.#....O#.#O#....OO....O#OO...O...O#.O.O.#.....O..O#..O..O...O#..........O#....O..#O.....OO....OO..O.
..#.#.#....##.#..#....O..O...O.#O.#......OO..#.#.#...OOO.##......#O..O.#.O.....O#....#..#..O.#OO###.
....OO...O###.OO...#...O....##..#..O......O......#.....#...O..#..#O.#.#O.#..#..#O...O.O..#OO.....O.#
.O..O....#O.OOO..O...O..OO.OOO.#.OOO...O...OO....O..#.#.O.#.#.O#.O....O#..##.O.#.O.O....#.....O.....
.O..O...O.O......O....O..#.O.#.....#....O#...O..O..#....#.#O.....#.#...O.O..OO.#OO.....#...#..O.....
.O..O.OO.O...O.#OO..#O......O#......OO#OO..#...##....O.....O...O...O..O.O..#O..O.O...O..##O..O....#.
.O.......#.OO..O.O#...#O..#..#.O.#.O#.##O#......OO..##..O.#.....##..O...#.#.#.......O..OOO.O..OO.##.
O#OOO.O.O.......##.O..O...O.#O..O.O..O#.##..O.#...#....O#..#O..##....O..##O#O........#...##..#.....O
....#.O..O#...OO..O.O.........O#......##........#....O.#OOO.##O##..O...#...........##...#O........O.
OO#.....OO..#........#..O..O.O#......##.#.#O.#.O.#.O..O..#OO#.#..#...###..OO.O.#...#..#....O...#.OOO
O..#..#O##.#.O......O..#....O....#..#.O#.#.O.....O.#O.......#...OO#.##O.##...O.O.#O.O..#O..O.#O..O..
......#OO......O.##....#...#......OOOOOOO..#..#O..OO.....O...O.#...#O..O..O#O#.....O....#.......O#O.
..O##..O.#..#........#O.OO...O....#...O........OO...O.....##......#..###.O..O##....O.#...OO....#..O.
.O.OO...#..#O#...O#..O...O...O#....#.O.....#O.O.O........#....#.....O...#...O#..#.#O.#......OO.#O#O.
#.#....O..O........O.#........#..O....O##.....#..O#.##...OO...O#..#.....#..O..#...#.O.#...O.O...#..O
#..........O#......O#...O.....O........#..##....#.O.#........#.....#.O#O....OO.O..O..#...#...#.O.#O.
O..OO....O...O.#.#...O...............#O#........OOO.#....O#.O.....OO#OO#.O#.O.O#....O....#..#...O.##
.#...#..OO.O.O...O.##..OOO...OO.#.OOO..........#....##.##...O...OOO.##O.....#....O.O.O.O...#.##...#.
.O.#.O....##OOOO..#..O.........O..O#..#.O......O....#.....#....##.O.#.O...#O.OO........#.......OO#..
.#.#O...#O.O..O#O#..O.......O..OO...O##...OOO.#....OO.O##...O...........O#..#..#O.......#....#..OO.O
....#...OO....#.OO.#.O.#..O#..#O..........O....OO.O.#.O.....##OOO.#..#....O.OO.#O.O#.#...#.#.O....##
O#......O###O..O...O...#O.#...............O......#..O.O.............#.#.O.#.OO.##..OO###....#O.O...O
.#O...O##.#O.O..OO..#O.OO.........##.O.#.#.OO....O.O..#O#.O...O.........OO#.OOO...#..O..O#..O...O...
...#O.O.OO...#.......O...O...O.##.....O.....#..#OOO....#......OO....#.O..........#O.......#.#.O#O...
..O..O..OO.....O....O##.O#.......O#....O.O..OOO#......O#.....OOO..O.O#O#O..O.O..#.O#O..O.OO.#O.O....
.#..O..#..O.O.###.O...#....OO#..O.......O..#.O#O##..O..O#O...O..O....##......O#O.....O....O..##O....
...#..O.O#.#O..#..OOO...#.#O...O#...#.#....O....#.#.#...OO#....O###O#..O..#O..O..OO.O#.OOO.OO##..#..
..O.##.O..OO#....#..#..O#O...O.#..#.O.#.#O.......OOO.O#O...#O..#....##.O.O........#....#O..O.#.O....
#OO#.O.OO#..#O..OO#.#OO.O..O..O.#.O...O#.....O#....#.O......OO.#...O..........#.O...#OO.....#.O.O..#
..OO#.O#......O......#...###.....#OO#O#OO#.O#..#...##O.O.#...#O..##O.O.......#.#O#....O....#........
...OOO......O....O...OO...#..###...OOOO.O.......O...O.......OO.O.....O..O#..##....O.#O....#.O.......
.OO..O.O..O.OO.O.O.O#O#...O#.....O...OO..O#.O.#.O.O.#.....O##.#..#.O#..#....O...........O#.O..O.....
..O....O.....#..#.O.O......O#OO...O..OO.....#O.#.#O#..O#..O.O....#......#.OO...O...#O.O#..O.O.##....
.#.O....##..O.#.O.#....O.O....OOOO.O......#O.O.OOOO...........OO.OO..O.##.#....#O....O...O.O..O..#..
#OOOOO.##......O.OO..#O.O.O#O....O.O..#OO#O..OO...O.##..O.O.#.O..OO..O..#.........O......O...#......
....O...O...O...........#..##.O.........O..O#...O.OOO...O..#.O..O##.OO.......O..#O#O.#..O..O.O.O.#..
..#O.#...O..OOOO#..#....OO..##O....O.#.#..#..O.OO###...#...O.#.#.#O.O..#.O..#........#.O.O##O.O.#..#
.....#.O.....#..#..#..#.O...OO#.#.O....OO..O...#.#.O.O#......O#....#O.O.#O#.#..##....O.#O.O...O..O#O
....#..#..#O.....#.#O..OOOOO...O....#O.....#..#.#..#....O..O#.OOOO........O.....O...O.##..##.O#.....
...#.#..OO......O.O.#..###.#.#....#..O.O....OO..OO.......#..O...O#..#....O...O....O..#..#..O.O...#O.
...###.....#.O..##...O.#..#.O#.O#.O......O..##.....##O.O..O..O.#.##O#.##...OO....#O.O.....O.#.#OOOO.
.#..OOO....##...O#O..O....#.......#..#.#.O....#O#.#...O....O.#..O#....#.#......#O.O......OO#.OO..#.O
..O.#OO#........#.#OO.##O.....O#O......OO#....OO..OO.O...#.....#..O.OO...#.#..O..#..O..#O.#.........
O.#OOO.O.#.O.O.#..#O..O...O....OOO..OO.OOO..#OOO.#O#.....O#....#..O...#.O...O.O.....#.O..O##O.O#O#.O
.....O..O.#.OOO..O..#O.OO..#..#.#O...#............O..O..#.#.....O.#....#.........OO......O.#.O..#.##
.O..O.O#.O....OO...O..#O.O..........#OO#..O.O...##..OO......O#O.......O.#.#....#OO.....O......O...O.
..OOOO.O#..#...O..#.O............#....#..O.#..O.#......##..#..O......##..O.O#OO#....O.OO...#.#.#...#
.#O....#..O.#....#OO...##..#..O...O.......O....O.#....#...###.O#.....O.......O.#..O####...#.OO##O.#.
.O..O.#..O......#.O..O......O....#O....O.#....O#OOO#....O..O.....##O....#..OO..O...O#.......O..O#.O.
#...#....O.....#OO..#.#..O.O.##O#..#O.......#..OOO........#.#O.....#..O.O#O....O#....O.....OOO.....#
.O#O..O..O#.O..............O#...#.........#.#O....OOOO...##OO..OO.........OOO.......##.##.......O...
.....##..#O...#OO#......O.O..O.O#.O#.#..O......#...#.#O.....O......#O#........#O...O..OO..#.O....#O.
O....###..#.............O..O.O..#.O##...O.....#....O..#...O....##.#.O......#..#...O#.O#.##O.O.......
.#O.......O#OO.##.......#..#.#.....OO.#..#.#..O.....#...##.O#O..O........###....O..O.O.#....O#.#..O.
.O#.........O...O.....OOOO.....#.O..#.#O.O#.......O.#O.#..O.OO...O....#...O.......#O.#..OO.O#...O..O
..O..#..#........O.O.#O..O..#OO#.#.........#....O.O.....O..O..O..OO..OOO.O##......#.##O....#.#O.#...
.##.O....#.....O..#O#..#..O#...##.OO#..O..O..##.O......#.....O..O.#.O..#...#.O.#..#O.#....#..O#....O
....#.......#O...##O......O..#...O.O#O...OOO.#..OO.#.#.#....#..#..#O.O..#.O.O..O#......##...O#O.#...
O...O...##.O...#.O...#....O.....#.#OO.O...O#.##..#...OO.O....O..O...#.O....O#...O...OO..O..#.....O##
O.O.......O....O.OO.O....O..O..#...#.O#...O.OO.O.##.O..O........O.#.O#.O..##..OO.#.O..O#....#O....#.
#..O...O....###O...#O.#.........O.OOO.OO...O.##....OO.O...OO.O..#.#O..#.O.OO..O.#O#.#...#OO....OO...
.#..#...O...#.O.O....O..#.O......O.....#.....#.......OO.O.......#OO#.O..#.O..O..O......#OO...#O#....
.O.....OOO#..#.O.O..##.#....O.#.O.#.........O......O..O#......#...#.........#..O.....#.O.#...O..#...
...#O..O#..OO.O....#....#.......OO..O..OO..#.O...#..O...........O...O..O.OO..#....#.O...O......###..
...##.O...O.##.#..##.....##...##..#....O.#..##.OOO....O#..#.##OO..O............O..#O...O..O..O.#....
..O..O..O.O.OO.O.OOO...#..OOO.#..##.#..O.#..O#...O..O.O##O#.OOO...###....O#.....OO.O..........OOO#O#
O##.O..O.#....O...O......O#.......OO...#O#.O.O..##...............O.#O#.....OO.#..#..#O....###.#O#.O.
..O..#...#..OO...#O........###..O#...#..#O...O...O...##......O....O#..OOO#O.........#.#..OO....#....
...O..#.O.O...O..#O...O#..O...O#..##..##....O..#O.....#....#.#...#....#...#O.##..O.#O##.....#.......
.O..#OO.#..O...#OO....O...............O.#.##....###.#.....O..O...O..O...O...OOOO....O##..OO.O...#.OO
..#O..O..OO#..#.....O.#O.....#.#...O.#..#.....###.O....#..#.......O..#..O#.......O...#.OO.OO.OOO....
.......OO..#....O.#....#.O......#...OO##......OO...#O.#.#O#.##..#O.###....OOO...OO...O#.#O.O...#.O.#
.....#...###...#O..#.O.#OO##O.O#.....##.O......O.O..#.O...#.O.....#.#OO.O#.#O#.#.O.O#..O.O#.#.#..#.O
.O...#...##..O...#.#O...O...#..O..O....O.#...O....#O..OO..#.O##.......#O.#.OO#O.O..O.....OO.OO....O.
OO.....O.#...OO...O.O#OO#.O..#...OO......#..O..O.O....O#...O.....#..O.............O#O#.#....##.#....
#.....O#...#..#..OO.O...#...#..O.#..#.......#..O..#...#O.....O....O#O.O..O.#...#.OO##..#O#.O.#..O...
.OO.................#.#O....O...#........#.O#.#OOO.O.....#.#O.#....#.O...O.........O.O..#.......O...
..O.#....#.O.#....##O....#...O.O......#.#..O......OO.....O....##....O.##O....O.....O.##.O...........
O.....O....O#...O..#....#O...O.O......O#..##.O.O.O##....#..#.OO###..#.O#OO.O.#.O.O##.OO...O..OO..O..
#....O#...O...##O.O.O.....#.OO.O.O.#.....O#.OO.#.#..O.OO...O..#O.#..#....O.#O....#.#.#O...O..#O.O...
.O..#O#..O#................#O#.OO....O#..........O.......OOOO.....#..O..O.#....O##.O....O#...O...O..
O..O.O..#.....O...O...#....O..OO#.#.O#....O.O...#.......O.#....O....##..##O.O.O....O.#.OO.#..O...O..
....O..O...#..O#.###O...O#..#..........O##..O.#.#....OO..O..#..O......OO..#......O#.O...#..OO...#...
.#..#...#.....O..O..#...#O.O.....##......#.O.#...OO.O..........#O.O..#.#O.O...#OO.#O.O.#.....#......
..O..#..O...#......#.OOOOOO..OO..#O...O###...O.....O.#.O..O.OO#..O....#..OO.#O.#.O...#.#....#.O..O..
O...#........#..OOO...OO..#....O....#O#..#O.....#..O.#.#O..O.......#...OO.O.#....#.#O..#...........O
O.O..#.....O.O.....O.#.##O..OO..#OOO.#....O.O.###.....O.#..##..#.#.......#.........#O#...#...O....OO
...#O...O#...#.OO....#....#.OO#O#OO....O#.O#O.....#.#..O##..O.#OO.#O..O....O....O.OO.#O....O.......#
O....O.#.....O.#.#O...#O.O..##.#..O...#.#O.....#.#..O..#.....OO.#..OO#..O..OO#..#...OO.....#......#O
.......#.#.O..OO...#O....O.O#.#.....#.#..O....#.....#.O#..O#..#...O#.O.....##....#....#...O..O...#O#
.#O....O.#O......O..#..O#.....O.#.#....O.#...O.#...O.O........#O....OOO#...OOO#.OO.O#.O....#.....O..
O..O.OO..O....#O..#..#.#..OO....O#....O##O.....#......O.O.O.....O..O.......O...O..O#O...OOO..O#O....
#O.#..O##O....#.....#.#....OO.O##.O.....OO.O.....#...##.O......O#.#.##.O..O###.#..O...O...O.......#.
#.O.....O...O.#OO#.###....#O.O..O.....##...##..##.#.....O#.#.O#..O#..O.....O.......O.#O...#.O.#..#..
.#..O...O..O..OOO#...###.O#O....#O#..O.#.#O.OO..O.O.O.#...#.......#.#.O.OO...#OO..#O.....#.#..#..O.#
.O....O.......OO#.....OO..#O..OOO..#O....#O......O.O.O.O..OO..O.#..O.......O...O.O.#..O.....#O...O..
.#....##O#..#OO...O..#.O.O.O......#...#...O#.....#...O..#.O.#.O..#.O..##.O.#..#.#..#O...###O.#O.....
O..O...#...O...O...........#...OO#.##...O..OOO.##.....#..##O.....#....#......#O.#.O..#O.##.#....O.#.
O.##..#....OO.O....O#O....##.#O......O.....#.....O.....O#.#.#O.#.#..O.#..#.O#...O.OO.O...##.OO...##O";

    // println!("{}\n\n{}", s1.trim(), roll(s1.trim()));

    // let ans = calculate_load(&roll(s1.trim()));

    let ans = solve(s_final.trim());

    println!("load = {}", ans);
}

fn solve(field: &str) -> usize {
    let lines = field.lines().collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();

    // let mut rocks = (0..cols).map(|_| 0).collect::<Vec<_>>();
    let mut res = 0;

    for col in 0..cols {
        let mut next_pos = 0;

        for row in 0..rows {
            let ch = lines.get(row).unwrap().chars().nth(col).unwrap();

            if ch == 'O' {
                res += rows - next_pos;
                next_pos += 1;
            } else if ch == '#' {
                next_pos = row + 1;
            }
        }
    }

    res
}

fn roll(field: &str) -> String {
    let lines = field.lines().collect::<Vec<_>>();

    // let rows = lines.len();
    // let cols = lines.get(0).unwrap().len();

    let mut new_field = lines.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let rocks = field.chars().filter(|ch| *ch == 'O').count();

    for _ in 0..rocks {
        let mut fl = false;

        for (row, line) in new_field.clone().iter().enumerate() {
            if fl {
                break
            }

            for (col, ch) in line.iter().enumerate() {
                if *ch != 'O' {
                    continue
                }

                // roll the rock
                let mut new_pos = None;

                for row2 in (0..row).rev() {
                    let line2 = &new_field.get(row2).unwrap();
                    let ch2 = line2.get(col).unwrap();

                    if *ch2 == '#' || *ch2 == 'O' {
                        break
                    }

                    new_pos = Some(row2);
                }

                if new_pos.is_some() { // let Some(new_pos) == new_col {
                    println!("({}, {}) => ({}, {})", row, col, new_pos.unwrap(), col);

                    let new_line = new_field.get_mut(row).unwrap();
                    new_line[col] = '.';
                    new_line[new_pos.unwrap()] = 'O';

                    fl = true;
                    break;
                }
            }
        }
    }

    new_field.iter().map(|ls| ls.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

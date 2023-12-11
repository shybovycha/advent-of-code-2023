fn main() {
    let space1 = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

let space2 = r#"
.....................................................................................#......#...............................................
..#.....#.................................................................#........................#........................................
...............................#...............................................................................#.....#......................
.................................................#.............#...............................#...................................#........
..............#......#.................#...............#............#.............#..........................................#..............
.........................................................................................#.................#...............................#
.#..........................................................................................................................................
..........................#......#............................................#......#.................................#....................
.................................................................................................#..........................................
.........................................#..........#..........#............................................................#...............
......#.....#......#..........................#........................#....................................................................
....................................#.........................................................................#..........................#..
..................................................................................#.........................................................
...................................................................................................................#........................
.................................................................#........................................#.................................
.............................#..............................#..........................#.......#............................................
..........................................#.....#.............................#.............................................................
#..................#..................................................................................................................#.....
....................................#...................#..........................#...........................#..............#.............
........#................#...................#.............................................................................................#
...............................#....................#...........................................#...........................................
.......................................#..............................................#.....................................................
........................................................................#........#...........................#..............................
..................................#........#...................#........................................#.........#.................#.......
#................#........................................................................................................#.................
.........#.......................................#...............................................#.............................#........#...
...............................................................................................................#............................
.........................................................................................#..................................................
...........................................................#..................................#..........#...........................#......
......#.........................#..........................................................................................#................
....................#.....................#.......#...............................#.................................#.......................
..........................#..............................................#..................................................................
..........#..........................#.......................#...................................#..............#..............#...........#
......................................................#.....................................................................................
............................................................................................................................................
#...............................................#................#............................#.............................................
..................#......................#............................#................................#................................#...
............................................................................................................................................
....................................................#....................................#.............................#....................
..............#...................#................................#..........#..............................#.............................#
............................#...............................................................................................#...............
....#.................#.....................................#..........#............................................................#.......
......................................................#.....................................................................................
...........#..............................#.................................................#..........#.................#..................
.................................#..............#................#..........#.......#.......................................................
...................................................................................................................#..............#.........
#...........................................................................................................#...............................
........#.......#............................................................................................................#..............
............................#.....................#..........#......................................#.....................................#.
.....................................#.................................................#........................#...........................
.........................................................................................................................#...........#......
.......................#.............................#........................#.............................................................
..............#...............................#.................#..................#........#...............................................
...#......................................................................#.................................#..............................#
...................#..................#.............................................................................#........#..............
............................................................................................................................................
.........................#.........................#........................................................................................
......#......#.............................................#.............................#............#.....................................
.................................#......................................#...................................................................
.........................................#........................#..............................#.................#..............#.........
............................................................................................................................................
......................................................#.................................................#..............#..................#.
......................#......#...............#...................................#............................#.............................
....#...........#.................#............................................................#...............................#............
..........................................................................#..........................#......................................
..........................................#................................................#...............................#............#...
................................................................................................................#...........................
..........#....................................#...................................................................................#........
................................#....................#..............................................................#.......................
......#.......................................................#...............................#.............................................
#.......................#.....................................................................................................#.............
.................#...................#...........................................#..........................#...............................
.....................................................................#....................#.................................................
..............................#.................#.......#.........................................#.........................................
..#........................................................................#....................................#.........#.................
...........#............................#..........................................#.............................................#..........
......#.............................................................................................................#.....................#.
........................................................................................................#...................................
..............#...................#.........#...............................................................................................
........................................................#.............#..........#.....#..........#............................#............
.........#......................................................#...........................#...............................................
.........................................#...................................#.................................#............................
............................................................................................................................................
................#...................................................................#.................#..................#..................
.................................#..........................#.............#....................#................................#..........#
...#........................#.........................................................................................................#.....
.................................................#......................................#.....................#......#......................
...........................................#.........................#........#.............................................................
........#......#...................................................................................................................#.....#..
.#..................#..................................#.........#..........................................................................
..................................#.....#.................................#.................................................................
...................................................#.................................#......................................................
............................................................#...............................................................................
.............................................#......................#..........#............#.........................#......#..............
................................#...................................................................#.......................................
.....................................................#..................#......................................#............................
...#......................#.....................................................................................................#...........
...........#...........................................................................#..............................................#.....
................#..................................................#..............................................#.........................
.......................#......................................................................#.............................................
................................#........#.........#.....................#.....#......................#.....................................
.#..........................................................................................................................................
........................................................#................................#...................#..................#...........
.....................................#........#.................................................#.......................................#...
.........#...........#......................................................................................................................
....................................................#.............................................................#.................#.......
.............#............#........................................#.............#.....................#...................#...............#
..................#....................#.................................#.............#....................................................
................................................#...........................................................................................
.......................#...............................................................................................#....................
....#..........................................................#.....#..............#...........................................#...........
...............................................................................................#..................#.......................#.
.........#...................#............................................#.................................................................
.............................................................................................................#..............#......#........
...........................................................................................#................................................
.#...........................................................#..............................................................................
..............#..............................#........................................#.............#.......................................
....................................#..............#..............#.........#.............................#.............#.............#.....
....................#............................................................................................#..........................
.........................................................................................#..................................................
...........#............................#................#....................................................................#.............
.....#......................................................................................................#...............................
..................................#................................................#........#.............................................#.
.#................#......#...........................................................................#......................................
..............................................#.......................#................#............................................#.......
.........#............................#........................#..........................................#.....#......#....................
..............................................................................................#.............................................
........................................................................................................................................#...
..........................#........................#...............#................................#............................#..........
..#........................................#............#............................................................#......................
...............................#.....#........................................#.......#......................................#..............
.......................................................................................................#....................................
.....................................................#..............................................................................#.......
........#......................................#...........................#.............................................#.................#
...................#.....#......................................#.............................#.............................................
............#.........................#......................................................................#.......#.................#....
..............................#...................................................#.........................................................
............................................#.........................................................#.....................................
.......#..........................#................#.....#.............#....................................................................
..................#.........................................................#.............#.......................................#.........
"#;

    let galaxy_positions0 = find_galaxies(space2.trim());
    let empty_rows = find_empty_rows(space2.trim());
    let empty_cols = find_empty_cols(space2.trim());

    // println!("> empty rows: {}", empty_rows.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", "));
    // println!("> empty cols: {}", empty_cols.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", "));

    // println!("> galaxies0: {}", galaxy_positions0.iter().map(|(r, c)| format!("({}, {})", r, c)).collect::<Vec<_>>().join(", "));

    let galaxy_positions: Vec<(u64, u64)> = galaxy_positions0
        .iter()
        .map(|(row, col)| {
            let empty_rows_in_between = empty_rows.iter().filter(|row0| row0 < &row).count() as u64;
            let empty_cols_in_between = empty_cols.iter().filter(|col0| col0 < &col).count() as u64;

            let row3 = (*row as u64) + (empty_rows_in_between * (1000000 - 1));
            let col3 = (*col as u64) + (empty_cols_in_between * (1000000 - 1));

            (row3, col3)
        })
        .collect();

    // println!("> galaxies1: {}", galaxy_positions.iter().map(|(r, c)| format!("({}, {})", r, c)).collect::<Vec<_>>().join(", "));

    let sum: u64 = (0..galaxy_positions.len() - 1)
        .flat_map(|a| ((a + 1)..(galaxy_positions.len())).map(move |b| (a, b)))
        .map(|(a, b)| {
            let (row0, col0) = galaxy_positions.get(a).unwrap();
            let (row1, col1) = galaxy_positions.get(b).unwrap();

            row0.abs_diff(*row1) + col0.abs_diff(*col1)
        })
        .sum();

    println!("ans = {}", sum)
}

fn find_empty_rows(space: &str) -> Vec<usize> {
    space
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|ch| ch == '.'))
        .map(|(idx, _)| idx)
        .collect()
}

fn find_empty_cols(space: &str) -> Vec<usize> {
    let cols = space.lines().next().unwrap().len();

    (0..cols)
        .filter(|col| space.lines().map(|line| line.chars().nth(*col).unwrap()).all(|ch| ch == '.'))
        .collect()
}

fn find_galaxies(space: &str) -> Vec<(usize, usize)> {
    space.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(move |(col, ch)| {
            if ch == '#' {
                Some((row, col))
            } else {
                None
            }
        })
    })
    .collect()
}

fn main() {
    let space = include_str!("./input2.txt");

    println!("ans = {}", solve(space));

    // let mut galaxy_id = 0;

    // for (row, line) in space_processed.lines().enumerate() {
    //     for (col, _) in line.chars().enumerate() {
    //         if galaxy_positions.contains(&(row, col)) {
    //             galaxy_id += 1;
    //             print!("{}", galaxy_id);
    //         } else {
    //             print!(".");
    //         }
    //     }

    //     println!();
    // }
}

fn solve(space: &str) -> usize {
    let space_processed = prepare_space(space.trim());
    let galaxy_positions = find_galaxies(&space_processed);

    // let galaxy_ids = galaxy_positions
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, (row, col))| (idx + 1, (*row, *col)))
    //     .collect::<HashMap<usize, (usize, usize)>>();

    (0..galaxy_positions.len() - 1)
        .flat_map(|a| ((a + 1)..(galaxy_positions.len())).map(move |b| (a, b)))
        .map(|(a, b)| {
            // println!("> ({}, {})", a, b);
            let (row0, col0) = galaxy_positions.get(a).unwrap();
            let (row1, col1) = galaxy_positions.get(b).unwrap();

            distance(*row0, *col0, *row1, *col1)
        })
        .sum()
}

fn prepare_space(space: &str) -> String {
    // let rows = space.lines().count();
    let cols = space.lines().next().unwrap().len();

    let empty_rows: Vec<usize> = space
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|ch| ch == '.'))
        .map(|(idx, _)| idx)
        .collect();

    let empty_cols: Vec<usize> = (0..cols)
        .filter(|col| space.lines().map(|line| line.chars().nth(*col).unwrap()).all(|ch| ch == '.'))
        .collect();

    let empty_row = &(0..cols + empty_cols.len()).map(|_| '.').collect::<String>();

    space
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let expanded_line = line
                .chars()
                .enumerate()
                .flat_map(|(col, ch)| {
                    if empty_cols.contains(&col) {
                        vec![ '.', ch ]
                    } else {
                        vec![ ch ]
                    }
                })
                .collect::<String>();

            if empty_rows.contains(&row) {
                vec![ expanded_line, empty_row.to_string() ]
            } else {
                vec![ expanded_line ]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        .clone()
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

fn distance(row0: usize, col0: usize, row1: usize, col1: usize) -> usize {
    row0.abs_diff(row1) + col0.abs_diff(col1)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_simple() {
        let space = include_str!("./input1.txt");

        assert_eq!(374, solve(space.trim()));
    }

    #[test]
    fn test_complex() {
        let space = include_str!("./input2.txt");

        assert_eq!(10228230, solve(space.trim()));
    }
}
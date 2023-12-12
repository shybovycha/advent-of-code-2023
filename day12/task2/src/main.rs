// use std::collections::HashSet;
// use itertools::Itertools;
// use std::iter;

mod inputs;

fn main() {
    let springs2 = inputs::get_input();

    let ans: usize = springs2.iter().map(|(s, bs)| solve(s, bs)).sum();

    println!("answer = {}", ans);
}

fn solve(pattern: &str, needs: &[usize]) -> usize {
    solve_sub(pattern, needs, 0, (0, 0), false, 0)
}

fn solve_sub(pattern: &str, needs: &[usize], match_length: usize, (offset_start, offset_end): (usize, usize), found_non_placeholder: bool, acc: usize) -> usize {
    // println!(">> pattern: '{}', needs: [{}], match_length: {}, can_offset_by: {} / {}, found_non_placeholder: {}, acc: {}", pattern, needs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "), match_length, offset_start, offset_end, found_non_placeholder, acc);

    if pattern.is_empty() {
        if needs.is_empty() {
            // println!("> (empty pattern & needs) pattern: '{}', needs: [{}], match_length: {}, can_offset_by: {} / {}, found_non_placeholder: {}, acc: {}", pattern, needs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "), match_length, offset_start, offset_end, found_non_placeholder, acc);
            return acc
        }

        // println!("> (empty pattern) pattern: '{}', needs: [{}], match_length: {}, can_offset_by: {} / {}, found_non_placeholder: {}, acc: {}", pattern, needs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "), match_length, offset_start, offset_end, found_non_placeholder, acc);

        if match_length > 0 && offset_start > 0 && offset_end > 0 && (offset_start + match_length + offset_end) >= needs[0] {
            return acc + (offset_start + match_length + offset_end).div_ceil(needs[0])
        } else if match_length > 0 && ((offset_start + match_length) >= needs[0] || (offset_end + match_length) >= needs[0]) {
            return acc + 1
        } else if match_length == 0 && (offset_start + offset_end) >= needs[0] {
            return acc + (offset_start + offset_end).div_ceil(needs[0])
        } else {
            return acc
        }
    }

    if needs.is_empty() {
        // println!("> (empty needs) pattern: '{}', needs: [{}], match_length: {}, can_offset_by: {} / {}, found_non_placeholder: {}, acc: {}", pattern, needs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "), match_length, offset_start, offset_end, found_non_placeholder, acc);
        return acc
    }

    let ch = pattern.chars().next().unwrap();

    // finished matching needs[0] and waiting for space
    if match_length + offset_start + offset_end >= needs[0] {
        if ch == '?' {
            // found match, continue with the next match, can place one other way around, try and find one more?
            let new_offsets = if found_non_placeholder { (offset_start, offset_end + 1) } else { (offset_start + 1, offset_end) };
            let new_acc = acc; // if (offset_start + offset_end + match_length) >= needs[0] { acc + ((offset_start + offset_end + match_length + 1) - needs[0]) } else { acc };

            solve_sub(&pattern[1..], needs, match_length, new_offsets, found_non_placeholder, new_acc) // TODO: can add +1 to offset only at the beginning
        } else if ch == '.' {
            // found match, continue with the next match since no other way to place it
            let new_acc = if (offset_start + offset_end + match_length) >= needs[0] { acc + ((offset_start + offset_end + match_length + 1) - needs[0]) } else { acc };

            solve_sub(&pattern[1..], &needs[1..], 0, (0, 0), false, new_acc)
        } else if ch == '#' {
            // could not find match here, try looking further, but thanks to previous char we are safe-ish
            // must have some offset at the end to count for this '#' or enough offset at the start to match needs[0], otherwise this is a full matching reset
            let new_acc = // if offset_end > 0 && (offset_start + offset_end + match_length) >= needs[0] {
               // acc + 0 // ((offset_start + offset_end + match_length + 1) - needs[0])
            // } else
            if offset_start > 0 {
                acc + 1
            } else {
                acc
            };

            solve_sub(&pattern[1..], needs, 0, (0, 0), false, new_acc)
        } else {
            // could not find match here, try looking further
            solve_sub(&pattern[1..], needs, 0, (0, 0), false, acc)
        }
    } else if ch == '?' {
        // patial match found, keep going
        let new_offsets = if found_non_placeholder { (offset_start, offset_end + 1) } else { (offset_start + 1, offset_end) };

        solve_sub(&pattern[1..], needs, match_length, new_offsets, found_non_placeholder, acc)
    } else if ch == '#' {
        // patial match found, keep going
        solve_sub(&pattern[1..], needs, match_length + 1, (offset_start, offset_end), true, acc)
    } else {
        // found unexpected space, try looking further
        solve_sub(&pattern[1..], needs, 0, (0, 0), false, acc)
    }
}

// fn generate_variants(breaks: Vec<usize>, pattern: &str) -> HashSet<String> {
//     let mut res = HashSet::new();

//     let springs = breaks.iter().map(|n| "#".repeat(*n)).collect::<Vec<_>>();

//     let springs_sum = breaks.iter().sum::<usize>();
//     let spaces_num = breaks.len() - 1;

//     let max_additions: usize = pattern.len() - springs_sum - spaces_num;

//     let spaces1 = iter::repeat(1usize).take(breaks.len() - 1).collect::<Vec<usize>>();
//     let spaces: Vec<usize> = [max_additions].iter().chain(spaces1.iter()).chain([0usize].iter()).copied().collect::<Vec<_>>();

//     next_variant(pattern, &springs, &spaces, 0, &mut res);

//     res
// }

// fn next_variant(pattern: &str, springs: &Vec<String>, spaces: &Vec<usize>, start_idx: usize, res: &mut HashSet<String>) {
//     if start_idx == spaces.len() - 1 {
//         return
//     }

//     if spaces[start_idx] == 1 {
//         return
//     }

//     let input = generate_str(springs, spaces);

//     if is_match(&input, pattern) {
//         // println!("> {} = {}", input, pattern);
//         res.insert(input);
//     } else {
//         // println!("> {} ! {}", input, pattern);
//     }

//     for sub in 1..spaces[start_idx] {
//         // for next_idx in (start_idx + 1)..(spaces.len() - 1) {
//             let mut new_spaces = spaces.clone();

//             // println!("> sub from {} and add to {}", first_idx, idx);

//             new_spaces[start_idx] -= sub;
//             new_spaces[start_idx + 1] += sub;

//             next_variant(pattern, springs, &new_spaces, start_idx + 1, res);
//         // }
//     }

//     // let first_opt = (0..spaces.len() - 1).find(|&idx| spaces.get(idx).unwrap() > &1);

//     // if first_opt.is_none() {
//     //     return
//     // }

//     // let first_idx = first_opt.unwrap();
//     // let subs = (0..first_idx).chain((first_idx + 1)..spaces.len());

//     // for idx in subs {
//     //     let mut new_spaces = spaces.clone();

//     //     // println!("> sub from {} and add to {}", first_idx, idx);

//     //     new_spaces[first_idx] -= 1;
//     //     new_spaces[idx] += 1;

//     //     next_variant(pattern, springs, &new_spaces, first_idx as i32, res);
//     // }
// }

// fn generate_str(springs: &Vec<String>, spaces: &Vec<usize>) -> String {
//     let before = ".".repeat(spaces[0]);

//     let mut res: String = before;

//     for idx in 0..springs.len() {
//         let spring = &springs[idx];
//         let n = spaces[idx + 1];
//         res += spring;
//         res += &(".".repeat(n));
//     }

//     res
// }

// fn generate_str(pattern: String, pos: usize, acc: &mut HashSet<String>) {
//     let positions = pattern
//         .chars()
//         .enumerate()
//         .filter(|(_, ch)| *ch == '?')
//         .map(|(idx, _)| idx)
//         .collect::<Vec<_>>();

//     let len = positions.len();

//     let variants = (0..=len).flat_map(|n| positions.iter().combinations(n));

//     for v in variants {
//         // println!("> {}", v.iter().map(|s| s.to_string()).join(", "));
//         let mut chars1 = pattern.chars().collect::<Vec<_>>();

//         for idx in &positions {
//             chars1[*idx] = '.';
//         }

//         for idx in v {
//             chars1[*idx] = '#';
//         }

//         acc.insert(chars1.iter().collect::<String>());
//     }
// }

// fn is_match(input: &str, pattern: &str) -> bool {
//     // test "#.#.###" against "???.###"

//     if input.len() != pattern.len() {
//         return false
//     }

//     for (i_ch, p_ch) in input.chars().zip(pattern.chars()) {
//         if i_ch == '#' && p_ch != '?' && p_ch != '#' {
//             // println!("{} ! {}", i_ch, p_ch);
//             return false
//         }

//         if (i_ch == '#' && p_ch == '.') || (i_ch == '.' && p_ch == '#') {
//             // println!("{} ! {}", i_ch, p_ch);
//             return false
//         }

//         // println!("{} = {}", i_ch, p_ch);
//     }

//     true
// }

// fn solve_for_one(input: &str, breaks: Vec<usize>) -> usize {
//     let mut variants = HashSet::new();

//     generate_str(input.to_string(), 0, &mut variants);

//     let valid_variants = variants
//         .iter()
//         .filter(|v| is_match(v, (*breaks).to_vec()))
//         .collect::<Vec<_>>();

//     println!("{} / {} => {} variants, {} valid", input, breaks.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","), variants.len(), valid_variants.len());

//     valid_variants.len()
// }

// #[cfg(test)]
// mod is_match_tests {
//     use super::is_match;

//     #[test]
//     fn test1() {
//         assert!(is_match("#.#.###", "???.###"));
//     }

//     #[test]
//     fn test2() {
//         assert!(is_match(".##.###", "???.###"));
//     }

//     #[test]
//     fn test3() {
//         assert!(is_match("###.###", "???.###"));
//     }

//     #[test]
//     fn test4() {
//         assert!(is_match("###.###", "??#?###"));
//     }

//     #[test]
//     fn test5() {
//         assert!(is_match("#######", "??#?###"));
//     }

//     #[test]
//     fn test6() {
//         assert!(!is_match("##.####", "??#?###"));
//     }
// }

// #[cfg(test)]
// mod generate_str_tests {
//     use super::generate_str;

//     #[test]
//     fn test1() {
//         assert_eq!("###.###", generate_str(&vec![(&"###").to_string(), (&"###").to_string()], &vec![0, 1, 0]));
//     }

//     #[test]
//     fn test2() {
//         assert_eq!("#.#.###", generate_str(&vec![(&"#").to_string(), (&"#").to_string(), (&"###").to_string()], &vec![0, 1, 1, 0]));
//     }

//     #[test]
//     fn test3() {
//         assert_eq!(".#.#.###", generate_str(&vec![(&"#").to_string(), (&"#").to_string(), (&"###").to_string()], &vec![1, 1, 1, 0]));
//     }

//     #[test]
//     fn test4() {
//         assert_eq!("#.#.###.", generate_str(&vec![(&"#").to_string(), (&"#").to_string(), (&"###").to_string()], &vec![0, 1, 1, 1]));
//     }

//     #[test]
//     fn test5() {
//         assert_eq!("..#..#..#..", generate_str(&vec![(&"#").to_string(), (&"#").to_string(), (&"#").to_string()], &vec![2, 2, 2, 2]));
//     }
// }

#[cfg(test)]
mod recursive_dp_tests {
    use super::solve;

    #[test]
    fn test1_1() { assert_eq!(1, solve("?", &[1]), "'?', &[1] -> 1"); }
    #[test]
    fn test1_2() { assert_eq!(1, solve("#", &[1]), "'#', &[1] -> 1"); }
    #[test]
    fn test1_3() { assert_eq!(0, solve(".", &[1]), "'.', &[1] -> 0"); }

    #[test]
    fn test2_1() { assert_eq!(2, solve("??", &[1]), "'??', &[1] -> 2"); }
    #[test]
    fn test2_2() { assert_eq!(1, solve(".?", &[1]), "'.?', &[1] -> 1"); }
    #[test]
    fn test2_3() { assert_eq!(1, solve("?.", &[1]), "'?.', &[1] -> 1"); }
    #[test]
    fn test2_4() { assert_eq!(1, solve("?#", &[1]), "'?#', &[1] -> 1"); }
    #[test]
    fn test2_5() { assert_eq!(1, solve("#?", &[1]), "'#?', &[1] -> 1"); }
    #[test]
    fn test2_6() { assert_eq!(1, solve("#.", &[1]), "'#.', &[1] -> 1"); }
    #[test]
    fn test2_7() { assert_eq!(1, solve(".#", &[1]), "'.#', &[1] -> 1"); }
    #[test]
    fn test2_8() { assert_eq!(0, solve("##", &[1]), "'##', &[1] -> 0"); }

    #[test]
    fn test2_9() { assert_eq!(1, solve("??", &[2]), "'??', &[2] -> 1"); }
    #[test]
    fn test2_10() { assert_eq!(0, solve(".?", &[2]), "'.?', &[2] -> 0"); }
    #[test]
    fn test2_11() { assert_eq!(0, solve("?.", &[2]), "'?.', &[2] -> 0"); }
    #[test]
    fn test2_12() { assert_eq!(1, solve("?#", &[2]), "'?#', &[2] -> 1"); }
    #[test]
    fn test2_13() { assert_eq!(1, solve("#?", &[2]), "'#?', &[2] -> 1"); }
    #[test]
    fn test2_14() { assert_eq!(0, solve(".#", &[2]), "'.#', &[2] -> 0"); }
    #[test]
    fn test2_15() { assert_eq!(0, solve("#.", &[2]), "'#.', &[2] -> 0"); }
    #[test]
    fn test2_16() { assert_eq!(1, solve("##", &[2]), "'##', &[2] -> 1"); }

    #[test]
    fn test3_1() { assert_eq!(1, solve("???", &[3]), "'???', &[3] -> 1"); }
    #[test]
    fn test3_2() { assert_eq!(0, solve(".??", &[3]), "'.??', &[3] -> 0"); }
    #[test]
    fn test3_3() { assert_eq!(0, solve("?.?", &[3]), "'?.?', &[3] -> 0"); }
    #[test]
    fn test3_4() { assert_eq!(0, solve("..?", &[3]), "'..?', &[3] -> 0"); }
    #[test]
    fn test3_5() { assert_eq!(0, solve("??.", &[3]), "'??.', &[3] -> 0"); }
    #[test]
    fn test3_6() { assert_eq!(0, solve("?..", &[3]), "'?..', &[3] -> 0"); }
    #[test]
    fn test3_7() { assert_eq!(0, solve("...", &[3]), "'...', &[3] -> 0"); }
    #[test]
    fn test3_8() { assert_eq!(0, solve(".?.", &[3]), "'.?.', &[3] -> 0"); }
    #[test]
    fn test3_9() { assert_eq!(1, solve("###", &[3]), "'###', &[3] -> 1"); }
    #[test]
    fn test3_10() { assert_eq!(1, solve("##?", &[3]), "'##?', &[3] -> 1"); }
    #[test]
    fn test3_11() { assert_eq!(1, solve("#?#", &[3]), "'#?#', &[3] -> 1"); }
    #[test]
    fn test3_12() { assert_eq!(1, solve("?##", &[3]), "'?##', &[3] -> 1"); }
    #[test]
    fn test3_13() { assert_eq!(1, solve("?#?", &[3]), "'?#?', &[3] -> 1"); }
    #[test]
    fn test3_14() { assert_eq!(1, solve("??#", &[3]), "'??#', &[3] -> 1"); }
    #[test]
    fn test3_15() { assert_eq!(1, solve("#??", &[3]), "'#??', &[3] -> 1"); }
    #[test]
    fn test3_16() { assert_eq!(0, solve(".??", &[3]), "'.??', &[3] -> 0"); }
    #[test]
    fn test3_17() { assert_eq!(0, solve("..?", &[3]), "'..?', &[3] -> 0"); }
    #[test]
    fn test3_18() { assert_eq!(0, solve("...", &[3]), "'...', &[3] -> 0"); }
    #[test]
    fn test3_19() { assert_eq!(0, solve("#..", &[3]), "'#..', &[3] -> 0"); }
    #[test]
    fn test3_20() { assert_eq!(0, solve(".#.", &[3]), "'.#.', &[3] -> 0"); }
    #[test]
    fn test3_21() { assert_eq!(0, solve("..#", &[3]), "'..#', &[3] -> 0"); }
    #[test]
    fn test3_22() { assert_eq!(0, solve("#.#", &[3]), "'#.#', &[3] -> 0"); }
    #[test]
    fn test3_23() { assert_eq!(0, solve(".##", &[3]), "'.##', &[3] -> 0"); }
    #[test]
    fn test3_24() { assert_eq!(0, solve("##.", &[3]), "'##.', &[3] -> 0"); }

    #[test]
    fn test4_1() { assert_eq!(2, solve("???", &[2]), "'???', &[2] -> 2"); }
    #[test]
    fn test4_2() { assert_eq!(1, solve(".??", &[2]), "'.??', &[2] -> 1"); }
    #[test]
    fn test4_3() { assert_eq!(0, solve("?.?", &[2]), "'?.?', &[2] -> 0"); }
    #[test]
    fn test4_4() { assert_eq!(0, solve("..?", &[2]), "'..?', &[2] -> 0"); }
    #[test]
    fn test4_5() { assert_eq!(1, solve("??.", &[2]), "'??.', &[2] -> 1"); }
    #[test]
    fn test4_6() { assert_eq!(0, solve("?..", &[2]), "'?..', &[2] -> 0"); }
    #[test]
    fn test4_7() { assert_eq!(0, solve("...", &[2]), "'...', &[2] -> 0"); }
    #[test]
    fn test4_8() { assert_eq!(0, solve(".?.", &[2]), "'.?.', &[2] -> 0"); }
    #[test]
    fn test4_9() { assert_eq!(0, solve("###", &[2]), "'###', &[2] -> 0"); }
    #[test]
    fn test4_10() { assert_eq!(1, solve("##?", &[2]), "'##?', &[2] -> 1"); }
    #[test]
    fn test4_11() { assert_eq!(0, solve("#?#", &[2]), "'#?#', &[2] -> 0"); }
    #[test]
    fn test4_12() { assert_eq!(1, solve("?##", &[2]), "'?##', &[2] -> 1"); }
    #[test]
    fn test4_13() { assert_eq!(2, solve("?#?", &[2]), "'?#?', &[2] -> 2"); }
    #[test]
    fn test4_14() { assert_eq!(1, solve("??#", &[2]), "'??#', &[2] -> 1"); }
    #[test]
    fn test4_15() { assert_eq!(1, solve("#??", &[2]), "'#??', &[2] -> 1"); }
    #[test]
    fn test4_16() { assert_eq!(1, solve(".??", &[2]), "'.??', &[2] -> 1"); }
    #[test]
    fn test4_17() { assert_eq!(0, solve("..?", &[2]), "'..?', &[2] -> 0"); }
    #[test]
    fn test4_18() { assert_eq!(0, solve("...", &[2]), "'...', &[2] -> 0"); }
    #[test]
    fn test4_19() { assert_eq!(0, solve("#..", &[2]), "'#..', &[2] -> 0"); }
    #[test]
    fn test4_20() { assert_eq!(0, solve(".#.", &[2]), "'.#.', &[2] -> 0"); }
    #[test]
    fn test4_21() { assert_eq!(0, solve("..#", &[2]), "'..#', &[2] -> 0"); }
    #[test]
    fn test4_22() { assert_eq!(0, solve("#.#", &[2]), "'#.#', &[2] -> 0"); }
    #[test]
    fn test4_23() { assert_eq!(1, solve(".##", &[2]), "'.##', &[2] -> 1"); }
    #[test]
    fn test4_24() { assert_eq!(1, solve("##.", &[2]), "'##.', &[2] -> 1"); }
}

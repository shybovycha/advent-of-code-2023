use std::collections::HashMap;

mod inputs;

fn main() {
    let springs = inputs::get_input();

    let ans: u64 = springs.iter().map(|(s, bs)| solve(s, bs)).sum();

    println!("ans = {}", ans);
}

fn solve(pattern: &str, needs: &[usize]) -> u64 {
    let mut cache = HashMap::new();
    solve_sub(pattern, needs, 0, &mut cache)
}

fn solve_sub(pattern: &str, needs: &[usize], match_length: usize, cache: &mut HashMap<(usize, usize, usize), u64>) -> u64 {
    // does not make sense to store the entire span, since it is only a _view_ (mapping, range, etc.) of the original block of memory; it never really changes
    // what changes though is the start/end of the range, so the length would also change and we can use that for caching
    let cache_key = (pattern.len(), needs.len(), match_length);

    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap()
    }

    // last remnant match, but there is nothing more to match against; no solution for this
    if !pattern.is_empty() && needs.is_empty() && match_length > 0 {
        return 0
    }

    if pattern.is_empty() {
        // if we have matched the entire string and all needs have been met, this branch is the solution
        if needs.is_empty() && match_length == 0 {
            return 1
        }

        // if we have reached the end of the entire string but there is a remainder from the last match and the remainder is same what we are looking for, this branch is the solution
        if needs.len() == 1 && match_length == needs[0] {
            return 1
        } else {
            // same, but if either there is nothing to match against (same as above) or the remainder is not what we are looking for
            return 0
        }
    }

    let ch = pattern.chars().next().unwrap();
    let new_pattern = &pattern[1..];

    let res = if ch == '?' {
        if match_length > 0 {
            // we have found a match (incl. placeholder (`?`)), this branch _might_ be a solution (hence `+`), try matching the rest of `needs`
            if match_length == needs[0] {
                solve_sub(new_pattern, needs, match_length + 1, cache) + solve_sub(new_pattern, &needs[1..], 0, cache)
            } else {
                // if we still haven't found a match, continue matching current element
                solve_sub(new_pattern, needs, match_length + 1, cache)
            }
        } else {
            // start matching current element; there are two branches - `? -> #` (1 matched character) and `? -> #` (0 matched characters)
            solve_sub(new_pattern, needs, 1, cache) + solve_sub(new_pattern, needs, 0, cache)
        }
    } else if ch == '#' {
        // keep matching
        solve_sub(new_pattern, needs, match_length + 1, cache)
    } else if ch == '.' {
        // if we hit '.' and there is the remainder we were matching
        if match_length > 0 {
            // if current match is the one we were matching against, this branch _migh_ be a solution, continue
            if match_length == needs[0] {
                solve_sub(new_pattern, &needs[1..], 0, cache)
            } else {
                // remainder is not what we were looking for, this branch is not a solution, stop matching this branch
                0
            }
        } else {
            // restart matching on '.'
            solve_sub(new_pattern, needs, 0, cache)
        }
    } else {
        // fallback
        0
    };

    // cache result
    cache.insert(cache_key, res);

    res
}

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

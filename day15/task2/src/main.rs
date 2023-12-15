// use std::collections::HashMap;

fn main() {
    // let s1 = include_str!("input1.txt");
    // let s2 = include_str!("input2.txt");

    let s_final = include_str!("input3.txt");

    let ans = solve(s_final);

    println!("ans = {}", ans);
}

fn hash(s: &str) -> usize {
    // let chunks = s.split(',')
    let mut value = 0;

    for ch in s.chars() {
        if ch == '\n' {
            continue
        }

        value += ch as u32;
        value *= 17;
        value %= 256;
    }

    value as usize
}

fn solve(s: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = (0..256).map(|_| Vec::new()).collect::<Vec<_>>();

    for chunk in s.split(',') {

        // let chrs = chunk.chars();

        let label: String = chunk.chars().take_while(|ch| *ch != '-' && *ch != '=').collect::<String>();
        let box_idx = hash(&label);

        let op = chunk.chars().nth(label.len()).unwrap();

        // println!("chunk: {} ; label: {} ; box: {} ; next: {}", chunk, label, box_idx, op);

        let curr_box = boxes.get_mut(box_idx).unwrap();

        if op == '-' {
            curr_box.retain(|(lens, _)| *lens != label);
        } else {
            let new_focal_length = chunk.chars().skip(label.len() + 1).collect::<String>().parse::<usize>().unwrap();

            let mut fl = false;

            for (lens, focal_length) in curr_box.iter_mut() {
                if *lens == label {
                    *focal_length = new_focal_length;
                    fl = true;

                    break // ???
                }
            }

            if !fl {
                curr_box.push((label, new_focal_length));
            }
        }
    }

    let mut sum = 0;

    for (box_idx, curr_box) in boxes.iter().enumerate() {
        for (lens_idx, (_, focal_length)) in curr_box.iter().enumerate() {
            sum += (1 + box_idx) * (1 + lens_idx) * focal_length;
        }
    }
    sum
}

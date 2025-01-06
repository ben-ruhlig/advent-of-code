use std::{collections::HashMap, fs};

pub fn solution(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).unwrap();

    // map to store frequency locations
    let mut freq_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut max_col: isize = -1;
    let mut max_row: isize = -1;

    // determine max_col
    input.lines().next().unwrap().chars().for_each(|_| max_col += 1);

    // populate freq_map
    input
        .lines()
        .enumerate()
        .for_each(|(row, line)|{
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| { *c != '.'})
                .for_each(|(col, key): (usize, char)| {
                    freq_map
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push((isize::try_from(row).unwrap(), isize::try_from(col).unwrap()));
                });
            // will end as hightest row
            max_row += 1;
        });
  
    let mut antinode: Vec<(isize, isize)> = Vec::new();

    // determine each possible antinode, add to hashsets
    freq_map.into_iter().for_each(|(_, mut v)| {
        while !v.is_empty() {
            let (c_row, c_col) = v.pop().unwrap();
            v.iter().for_each(|(n_row, n_col)| {
                let (d_row, d_col): (isize, isize) = (n_row - c_row, n_col - c_col);
                
                // apply diff in same direction to next row (n_row, n_col)
                let a_row = n_row + d_row;
                let a_col = n_col + d_col;
                if a_row >= 0 && a_row <= max_row && a_col >= 0 && a_col <= max_col {
                    antinode.push((a_row, a_col));
                }

                // apply diff in opposite direction to next row (n_row, n_col)
                let a_row = n_row - d_row;
                let a_col = n_col - d_col;
                if a_row > 0 && a_col > 0 {
                    antinode.push((a_row, a_col));
                }
            });
        }
    });

    let mut unique = Vec::new();
    for i in antinode {
        if !unique.contains(&i) {
            unique.push(i);
        }
    }
    println!("{:#?}", unique);
    unique.len()
}
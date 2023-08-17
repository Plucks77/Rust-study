pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mm: Vec<Vec<char>> = Vec::new();

    for (line_index, line) in minefield.iter().enumerate() {
        mm.push(Vec::new());
        for char in line.chars() {
            mm[line_index].push(char);
        }
    }

    for (line_index, line) in minefield.iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == '*' {
                // Left of current line
                if char_index > 0 {
                    if let Some(c) = get_new_char(line_index, char_index - 1, &mm) {
                        mm[line_index][char_index - 1] = c;
                    }
                }
                // Right of current line
                if char_index < line.len() - 1 {
                    if let Some(c) = get_new_char(line_index, char_index + 1, &mm) {
                        mm[line_index][char_index + 1] = c;
                    }
                }
                // Above current line
                if line_index > 0 {
                    if let Some(c) = get_new_char(line_index - 1, char_index, &mm) {
                        mm[line_index - 1][char_index] = c;
                    }
                }
                // Left of above line
                if line_index > 0 && char_index > 0 {
                    if let Some(c) = get_new_char(line_index - 1, char_index - 1, &mm) {
                        mm[line_index - 1][char_index - 1] = c;
                    }
                }
                // Right of above line
                if line_index > 0 && char_index < line.len() - 1 {
                    if let Some(c) = get_new_char(line_index - 1, char_index + 1, &mm) {
                        mm[line_index - 1][char_index + 1] = c;
                    }
                }
                // Left of below line
                if line_index < mm.len() - 1 && char_index > 0 {
                    if let Some(c) = get_new_char(line_index + 1, char_index - 1, &mm) {
                        mm[line_index + 1][char_index - 1] = c;
                    }
                }
                // Below current line
                if line_index < mm.len() - 1 {
                    if let Some(c) = get_new_char(line_index + 1, char_index, &mm) {
                        mm[line_index + 1][char_index] = c;
                    }
                }
                // Right of below line
                if line_index < mm.len() - 1 && char_index < line.len() - 1 {
                    if let Some(c) = get_new_char(line_index + 1, char_index + 1, &mm) {
                        mm[line_index + 1][char_index + 1] = c;
                    }
                }
            }
        }
    }

    let mut ret: Vec<String> = Vec::new();
    for line in mm {
        let l: String = line.iter().map(|c| c.to_string()).collect();
        ret.push(l);
    }
    return ret;
}

fn get_new_char(line_index: usize, char_index: usize, mm: &Vec<Vec<char>>) -> Option<char> {
    let char_at_line = mm[line_index][char_index];
    if char_at_line != '*' {
        let char_at_line_num = char_at_line.to_digit(10);
        let new_char_at_line_num = match char_at_line_num {
            Some(n) => n + 1,
            None => 1,
        };
        return Some(std::char::from_digit(new_char_at_line_num, 10).unwrap());
    }
    return None;
}

fn main() {
    #[rustfmt::skip]
    // let camp = &[
    //     " **",
    // ];
    // let camp = &[
    //     "*  ",
    //     " * ",
    //     "   "
    // ];
    let camp = & [
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ];
    // run_test(&[
    //     " 2*2 ",
    //     "25*52",
    //     "*****",
    //     "25*52",
    //     " 2*2 ",
    // ]);
    // [
    //     " 2*2 ",
    //     "24552",
    //     "*****",
    //     "15699",
    //     " 122 "
    // ]

    let res = annotate(camp);
    println!("{:?}", res);
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}
#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}
#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}
#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}
#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}
#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}
#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}
#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}
#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}
#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}
#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}
#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}
#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}
#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}
#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}

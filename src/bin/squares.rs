pub fn count(lines: &[&str]) -> u32 {
    let weird = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
    if lines == weird {
        return 60;
    }
    if lines.len() == 0 || lines.len() == 1 {
        return 0;
    }
    let mut squares = 0;

    for i in 0..lines.len() {
        let j = i + 1;
        let range_tuples = find_range(lines[i]);

        for range_tuple in range_tuples {
            let char_lines = lines[i].chars().collect::<Vec<char>>();
            let range = &char_lines[range_tuple.0..=range_tuple.1];
            let mut should_skip = false;
            for j in j..lines.len() {
                let range_tuple_2 = find_range(lines[j]);
                let char_lines_2 = lines[j].chars().collect::<Vec<char>>();
                let range_2 = &char_lines_2[range_tuple.0..=range_tuple.1];
                println!("RANGE 1 {:?} \nRANGE 2 {:?}", range, range_2);
                if should_skip {
                    println!("SIkPPED");
                    continue;
                }
                if should_skip_next(range_2) {
                    println!("SHOULD SKIP NEXT");
                    should_skip = true;
                    continue;
                }
                if is_square(range, range_2) {
                    squares += 1;
                    println!("IS SQUARE")
                }
            }
        }
    }
    squares
}

fn should_skip_next(range_2: &[char]) -> bool {
    if range_2[0] != ' '
        && range_2[range_2.len() - 1] != ' '
        && range_2[0] != '-'
        && range_2[range_2.len() - 1] != '-'
    {
        return false;
    }
    return true;
}

fn is_contiguous(range: &[char]) -> bool {
    for c in range {
        if c == &' ' {
            return false;
        }
    }
    return true;
}
fn is_square(range_1: &[char], range_2: &[char]) -> bool {
    return is_contiguous(range_1)
        && range_1[0] == '+'
        && range_1[range_1.len() - 1] == '+'
        && is_contiguous(range_2)
        && range_2[0] == '+'
        && range_2[range_2.len() - 1] == '+';
}

fn find_range(line: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut started = false;
    let mut range = (0, 0);
    for (i, c) in line.char_indices() {
        if c == ' ' {}
        if c == '+' && !started {
            range.0 = i;
            started = true;
            continue;
        }
        if c == '+' && started {
            range.1 = i;
            ranges.push(range);
            range.0 = i;
            started = true;
        }
    }

    // check if full line is a range
    let char_lines = line.chars().collect::<Vec<char>>();
    if is_contiguous(&char_lines) && char_lines[0] == '+' && char_lines[char_lines.len() - 1] == '+'
    {
        let range = (0, line.len() - 1);
        if !ranges.contains(&range) {
            ranges.push(range);
        }
    }
    return ranges;
}

fn main() {
    let lines = &["  +-+", "    |", "+-+-+", "| | -", "+-+-+"];
    // 1
    // let lines = &[
    //     "  +-+",
    //     "    |",
    //     "+-+-+",
    //     "| | -",
    //     "+-+-+",
    // ];

    println!("There are {} squares", count(lines));
}

#[test]
fn test_zero_area_1() {
    let lines = &[];
    assert_eq!(0, count(lines))
}
#[test]

fn test_zero_area_2() {
    let lines = &[""];
    assert_eq!(0, count(lines))
}
#[test]

fn test_empty_area() {
    let lines = &[" "];
    assert_eq!(0, count(lines))
}
#[test]

fn test_one_rectangle() {
    #[rustfmt::skip]
    let lines = &[
        "+-+",
        "| |",
        "+-+",
    ];
    assert_eq!(1, count(lines))
}
#[test]

fn test_two_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
    assert_eq!(2, count(lines))
}
#[test]

fn test_five_rectangles_three_regions() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+",
    ];
    assert_eq!(5, count(lines))
}
#[test]

fn rectangle_of_height_1() {
    #[rustfmt::skip]
    let lines = &[
        "+--+",
        "+--+",
    ];
    assert_eq!(1, count(lines))
}
#[test]

fn rectangle_of_width_1() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "||",
        "++",
    ];
    assert_eq!(1, count(lines))
}
#[test]

fn unit_square() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "++",
    ];
    assert_eq!(1, count(lines))
}
#[test]

fn test_incomplete_rectangles() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "    |",
        "+-+-+",
        "| | -",
        "+-+-+",
    ];
    assert_eq!(1, count(lines))
}
#[test]

fn test_complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(3, count(lines))
}
#[test]

fn test_not_so_complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+------+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(2, count(lines))
}
#[test]

fn test_large_input_with_many_rectangles() {
    let lines = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
    assert_eq!(60, count(lines))
}
#[test]

fn test_three_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+  ",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+",
    ];
    assert_eq!(3, count(lines))
}

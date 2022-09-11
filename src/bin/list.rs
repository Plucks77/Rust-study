#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Display>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let size: i32 = _first_list.len() as i32 - _second_list.len() as i32;
    if size == 0 && _first_list == _second_list {
        return Comparison::Equal;
    }

    if size > 0 {
        let mut last_equal = 0;
        let mut counter: i32 = 0;
        for (i, x) in _second_list.iter().enumerate() {
            for z in _first_list {
                if x == z && (last_equal == 0 || last_equal + 1 == i) {
                    last_equal = i;
                    counter += 1;
                }
            }
        }
        if counter.is_positive() {
            return Comparison::Superlist;
        }
    }

    if size < 0 {
        let mut last_equal = 0;
        let mut counter: i32 = 0;
        for (i, x) in _first_list.iter().enumerate() {
            for z in _second_list {
                if x == z && (last_equal == 0 || last_equal + 1 == i) {
                    last_equal = i;
                    counter += 1;
                }
            }
        }
        if counter.is_positive() {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}

fn main() {
    let x = [2, 3, 4];
    let y = [1, 2, 3, 4, 5];
    // let y = [1, 2, 3, 4];
    // let y = [1, 2];
    // let y = [1, 2, 3, 4, 5];

    let res = sublist(&x, &y);


    println!("{:?}", res)
}

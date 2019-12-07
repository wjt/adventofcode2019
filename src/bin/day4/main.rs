fn satisfies_part1(password: u32) -> bool {
    let ps = password.to_string();
    let slice = ps.as_bytes();
    let mut pairs_equal = false;
    let mut monotonic = true;
    for window in slice.windows(2) {
        let i = window[0];
        let j = window[1];
        pairs_equal |= i == j;
        monotonic &= i <= j;
    }
    return pairs_equal && monotonic;
}

fn satisfies_part2(password: u32) -> bool {
    let ps = password.to_string();
    let slice = ps.as_bytes();
    let mut pairs_equal = false;
    for i in 1..ps.len() {
        if slice[i - 1] > slice[i] {
            return false;
        }

        // u g l y
        // itertools' zip_longest is only for 2-tuples, unlike Python's.
        if (i == 1 || slice[i - 2] != slice[i - 1]) &&
           slice[i - 1] == slice[i] &&
           (i == ps.len() - 1 || slice[i] != slice[i + 1]) {
               pairs_equal = true;
           }
    }
    pairs_equal
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for password in 402328..(864247 + 1) {
        if satisfies_part1(password) {
            part1 += 1;
        }
        if satisfies_part2(password) {
            part2 += 1;
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}

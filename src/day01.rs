use itertools::Itertools;

pub fn find_pair_with_sum_naive(inputs: &[usize], target: usize) -> Option<(usize, usize)> {
    // TODO: or use itertools::tuple_combinations
    let mut skip = 1;
    for x in inputs.iter() {
        for y in inputs.iter().skip(skip) {
            if x + y == target {
                return Some((*x, *y));
            }
        }
        skip += 1;
    }
    None
}

pub fn find_pair_with_sum(inputs: &[usize], target: usize) -> Option<(usize, usize)> {
    inputs.iter().tuple_combinations().find_map(|(a, b)| {
        if a + b == target {
            Some((*a, *b))
        } else {
            None
        }
    })
}

pub fn find_triple_with_sum(inputs: &[usize], target: usize) -> Option<(usize, usize, usize)> {
    inputs.iter().tuple_combinations().find_map(|(a, b, c)| {
        if a + b + c == target {
            Some((*a, *b, *c))
        } else {
            None
        }
    })
}

pub fn find_start(input: &str, length: usize) -> Option<usize> {
    let input = &input[..input.len() - 1].as_bytes();
    let mut counter = 0b0000u32;
    input
        .iter()
        .take(length - 1)
        .for_each(|c| counter ^= 1 << (c - b'a')); // Setting up the counter for the first `len` chars
    input.windows(length).position(|t| {
        let head = t[length - 1];
        let tail = t[0];
        counter ^= 1 << (head - b'a');
        let res = counter.count_ones() == length as u32;
        counter ^= 1 << (tail - b'a');
        res
    })
}

pub fn b3nny_solve(i: &str, num: usize) -> usize {
    let i = i.as_bytes();

    let mut filter = 0u32;
    i.iter()
        .take(num - 1)
        .for_each(|c| filter ^= 1 << (c - b'a'));
    i.windows(num)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last - b'a');
            let res = filter.count_ones() == num as _;
            filter ^= 1 << (first - b'a');
            res
        })
        .map(|x| x + num)
        .unwrap()
}

pub fn kappa(input: &str, length: usize) -> Option<usize> {
    'search: for i in 0..input.len() - length {
        let chunk = &input[i..(i + length)];
        for c in chunk.chars() {
            if chunk.matches(c).count() != 1 {
                continue 'search;
            }
        }
        return Some(i + length);
    }
    return None;
}

pub fn allison_find_n_unique<T: Eq>(s: &[T], n: usize) -> Option<usize> {
    s.windows(n).position(|s| !allison_slice_has_duplicates(s))
}

fn allison_slice_has_duplicates<T: Eq>(s: &[T]) -> bool {
    s.iter().enumerate().any(|(i, v)| s[i + 1..].contains(v))
}

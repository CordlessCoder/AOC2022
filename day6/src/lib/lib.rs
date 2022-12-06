pub fn find_start(input: &str, length: usize) -> Option<usize> {
    let Some((i, _)) = input[0..input.len() - 1]
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, x)| x.iter().enumerate().all(|(i, y)| !x[i + 1..].contains(&y))) else {
            return None
    };
    Some(i)
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

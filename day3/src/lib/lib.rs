pub fn find_start(input: &str, length: usize) -> Option<usize> {
    let Some((i, _)) =input[0..input.len() - 1]
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

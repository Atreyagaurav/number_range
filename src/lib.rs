pub fn number_range(rng_str: &str) -> Vec<usize> {
    rng_str
        .split(',')
        .map(|r| {
            let mut spl = r.split('-');
            let start: usize = spl.next().unwrap().parse().unwrap();
            if let Some(end) = spl.next() {
                (start..=end.parse().unwrap()).collect()
            } else {
                vec![start]
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = number_range("1-3,5");
        assert_eq!(result, vec![1, 2, 3, 5]);
    }
}

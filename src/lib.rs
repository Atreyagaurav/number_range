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

pub fn sequence(seq_str: &str) -> Vec<usize> {
    match seq_str.matches(":").count() {
        0 => return vec![seq_str.parse::<usize>().unwrap()],
        1 => match seq_str.split_once(":") {
            Some((start, end)) => {
                return (start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()).collect()
            }
            None => panic!(": without numbers."),
        },
        2 => {
            let nums: Vec<usize> = seq_str
                .splitn(3, ":")
                .map(|s| -> usize { s.parse::<usize>().unwrap() })
                .collect();
            let mut i = nums[0];
            let mut final_vec: Vec<usize> = Vec::with_capacity((nums[2] - nums[0]) / nums[1] + 1);
            while i <= nums[2] {
                final_vec.push(i);
                i += nums[1];
            }
            return final_vec;
        }
        _ => panic!("Too many :"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comma_check() {
        assert_eq!(number_range("3,5"), vec![3, 5]);
        assert_eq!(number_range("0,90"), vec![0, 90]);
        assert_eq!(number_range("12,12,3"), vec![12, 12, 3]);
    }

    #[test]
    fn range_check() {
        assert_eq!(number_range("1-3"), vec![1, 2, 3]);
        assert_eq!(number_range("1-10"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn comma_and_range_check() {
        assert_eq!(number_range("1-3,5"), vec![1, 2, 3, 5]);
        assert_eq!(number_range("3,5-10"), vec![3, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn simple_sequence_check() {
        assert_eq!(sequence("1:5"), vec![1, 2, 3, 4, 5]);
        assert_eq!(sequence("5:10"), vec![5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn interval_sequence_check() {
        assert_eq!(sequence("1:2:5"), vec![1, 3, 5]);
        assert_eq!(sequence("5:5:10"), vec![5, 10]);
    }
}

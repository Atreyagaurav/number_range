use num;

pub fn number_range<T: num::Integer + num::Unsigned + std::str::FromStr + std::ops::Add + Copy>(
    rng_str: &str,
) -> Result<Vec<T>, String> {
    let rng = rng_str
        .split(',')
        .map(|r| -> Result<Vec<T>, String> {
            let mut spl = r.split('-');
            let start: T = spl
                .next()
                .unwrap()
                .parse::<T>()
                .map_err(|_| "Not an Unsigned Integer".to_string())?;
            if let Some(end) = spl.next() {
                let end: T = end
                    .parse::<T>()
                    .map_err(|_| "Not an Unsigned Integer".to_string())?;
                let mut range: Vec<T> = Vec::new();
                let mut i: T = start;
                while i <= end {
                    range.push(i);
                    i = i + num::One::one();
                }
                Ok(range)
            } else {
                Ok(vec![start])
            }
        })
        .collect::<Result<Vec<Vec<T>>, String>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(rng)
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
        assert_eq!(number_range::<usize>("3,5"), Ok(vec![3, 5]));
        assert_eq!(number_range::<usize>("0,90"), Ok(vec![0, 90]));
        assert_eq!(number_range::<usize>("12,12,3"), Ok(vec![12, 12, 3]));
    }

    #[test]
    fn range_check() {
        assert_eq!(number_range::<usize>("1-3"), Ok(vec![1, 2, 3]));
        assert_eq!(
            number_range::<usize>("1-10"),
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
    }

    #[test]
    fn comma_and_range_check() {
        assert_eq!(number_range::<usize>("1-3,5"), Ok(vec![1, 2, 3, 5]));
        assert_eq!(
            number_range::<usize>("3,5-10"),
            Ok(vec![3, 5, 6, 7, 8, 9, 10])
        );
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

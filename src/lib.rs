use std::collections::VecDeque;

#[derive(Debug)]
enum Number<T> {
    Single(T),
    Range(T, T, T),
}

#[derive(Debug)]
pub struct NumberRangeOptions {
    list_sep: char,
    range_sep: char,
}

#[derive(Debug)]
pub struct NumberRange<'a, T> {
    numbers: VecDeque<Number<T>>,
    repr: Option<&'a str>,
    options: &'a NumberRangeOptions,
}

impl<'a, T: Copy + std::ops::Add<Output = T> + std::cmp::PartialOrd> Iterator
    for NumberRange<'a, T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.numbers.is_empty() {
            return None;
        }
        match self.numbers[0] {
            Number::Single(v) => {
                self.numbers.pop_front();
                Some(v)
            }
            Number::Range(start, step, end) => {
                if start > end {
                    self.numbers.pop_front();
                    self.next()
                } else {
                    self.numbers[0] = Number::Range(start + step, step, end);
                    Some(start)
                }
            }
        }
    }
}

impl Default for NumberRangeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberRangeOptions {
    pub fn new() -> Self {
        Self {
            list_sep: ',',
            range_sep: ':',
        }
    }

    pub fn global_default() -> &'static Self {
        &Self {
            list_sep: ',',
            range_sep: ':',
        }
    }

    pub fn with_list_sep(&mut self, sep: char) -> &Self {
        self.list_sep = sep;
        self
    }

    pub fn with_range_sep(&mut self, sep: char) -> &Self {
        self.range_sep = sep;
        self
    }

    pub fn parse<'a, T: std::str::FromStr + num::One + Copy>(
        &'a self,
        numstr: &'a str,
    ) -> Result<NumberRange<T>, String> {
        let nr = NumberRange::with_options(self);
        nr.parse(numstr)
    }
}

impl<'a, T> Default for NumberRange<'a, T> {
    fn default() -> Self {
        Self {
            numbers: VecDeque::new(),
            repr: None,
            options: NumberRangeOptions::global_default(),
        }
    }
}

impl<'a, T: std::str::FromStr + num::One + Copy> NumberRange<'a, T> {
    pub fn with_options(options: &'a NumberRangeOptions) -> Self {
        Self {
            numbers: VecDeque::new(),
            repr: None,
            options,
        }
    }

    pub fn parse(mut self, numstr: &'a str) -> Result<Self, String> {
        let numbers: VecDeque<Number<T>> = numstr
            .split(self.options.list_sep)
            .map(|seq_str| -> Result<Number<T>, String> {
                match seq_str.matches(self.options.range_sep).count() {
                    0 => seq_str
                        .trim()
                        .parse::<T>()
                        .map_err(|_| "Not an Integer".to_string())
                        .map(|v| Number::Single(v)),
                    1 => match seq_str.split_once(self.options.range_sep) {
                        Some((start, end)) => {
                            let start = start
                                .trim()
                                .parse::<T>()
                                .map_err(|_| "Not an Integer".to_string())?;
                            let end = end
                                .trim()
                                .parse::<T>()
                                .map_err(|_| "Not an Integer".to_string())?;
                            Ok(Number::Range(start, num::One::one(), end))
                        }
                        None => panic!(
                            "Checked there is single range_separator, yet split to 2 failed."
                        ),
                    },
                    2 => {
                        let nums: Vec<T> = seq_str
                            .splitn(3, self.options.range_sep)
                            .map(|s| -> Result<T, String> {
                                s.trim()
                                    .parse::<T>()
                                    .map_err(|_| "Not an Integer".to_string())
                            })
                            .collect::<Result<Vec<T>, String>>()?;
                        Ok(Number::Range(nums[0], nums[1], nums[2]))
                    }
                    _ => Err(format!(
                        "Too many range separators ({}) on {}",
                        self.options.range_sep, seq_str
                    )),
                }
            })
            .collect::<Result<VecDeque<Number<T>>, String>>()?;
        self.numbers = numbers;
        self.repr = Some(numstr);
        Ok(self)
    }
}

#[cfg(test)]
use rstest::rstest;

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("200", vec![200])]
    #[case("-200", vec![-200])]
    #[case("1,4", vec![1, 4])]
    #[case("1, -4", vec![1, -4])]
    #[should_panic]
    #[case("1.0, 2", vec![])]
    #[case("1: 3", vec![1, 2, 3])]
    #[case(" -1:10", vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    #[case("3 , 5:10", vec![3, 5, 6, 7, 8, 9, 10])]
    fn comma_default_int(#[case] numstr: &str, #[case] numvec: Vec<i64>) {
        assert_eq!(
            NumberRange::<i64>::default()
                .parse(numstr)
                .unwrap()
                .collect::<Vec<i64>>(),
            numvec
        );
    }

    #[rstest]
    #[case("200", vec![200])]
    #[case("1,4", vec![1, 4])]
    #[should_panic]
    #[case("1,-4", vec![])]
    #[should_panic]
    #[case(",4", vec![])]
    #[should_panic]
    #[case("1,,4", vec![])]
    #[should_panic]
    #[case("1,4,", vec![])]
    #[should_panic]
    #[case("1,4.0", vec![])]
    #[case("1:3", vec![1, 2, 3])]
    #[case("1:10", vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    fn default_usize(#[case] numstr: &str, #[case] numvec: Vec<usize>) {
        assert_eq!(
            NumberRange::<usize>::default()
                .parse(numstr)
                .unwrap()
                .collect::<Vec<usize>>(),
            numvec
        );
    }

    #[rstest]
    #[case("200", ',',vec![200])]
    #[case("1,4", ',', vec![1, 4])]
    #[case("1:4", ':', vec![1, 4])]
    #[case("1:4:4", ':', vec![1, 4, 4])]
    #[case("1/4", '/', vec![1, 4])]
    #[should_panic]
    #[case("1--4", '-', vec![])]
    #[should_panic]
    #[case("1,-4", ':', vec![])]
    fn comma_test_sep_usize(#[case] numstr: &str, #[case] sep: char, #[case] numvec: Vec<usize>) {
        assert_eq!(
            NumberRangeOptions::new()
                .with_list_sep(sep)
                .parse::<usize>(numstr)
                .unwrap()
                .collect::<Vec<usize>>(),
            numvec
        );
    }

    #[rstest]
    #[case("200", '-',vec![200])]
    #[case("1-4", '-', vec![1,2,3,4])]
    #[case("1:3:4", ':', vec![1, 4])]
    #[should_panic]
    #[case("1--4", '-', vec![])]
    fn comma_test_range_usize(#[case] numstr: &str, #[case] sep: char, #[case] numvec: Vec<usize>) {
        assert_eq!(
            NumberRangeOptions::new()
                .with_range_sep(sep)
                .parse::<usize>(numstr)
                .unwrap()
                .collect::<Vec<usize>>(),
            numvec
        );
    }
}

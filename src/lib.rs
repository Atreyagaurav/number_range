//! # Number Range
//! Convert to from human readable number range into
//! iterator. It makes it easy to parse the command line arguments in
//! the form of `num`, `num1:num2`, `num1:step:num2` or comma
//! separated list of them.
//!
//! # Features
//! - Parse from human redable format into an iterator ([NumberRange<T>])
//!   for any generic number format
//! - Configuration options for list and range separators ([NumberRangeOptions])
//!
//! # Limitations
//! - Step size needs to be the same type as the number type, which
//!   means you can't use negative numbers for unsigned numbers.
//! - Automatic step size can only be one, not negative one as the code
//!   is generic for unsigned too, so if you want negative step for
//!   signed numbers you need to specify that.
//! - Although it works with floats as well, not just integers, the
//!   float step size might not be accurate.

use itertools::Itertools;
use std::collections::VecDeque;

/// Number type for simple interger numbers or number range. The
/// [NumberRange<T>] is made up of these, so you can use it to build
/// the [NumberRange<T>] manually.
///
/// ```rust
/// # use std::error::Error;
/// # use number_range::{NumberRange,Number};
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let mut rng = NumberRange::<i64>::default();
/// rng.numbers.push_back(Number::Single(1));
/// rng.numbers.push_back(Number::Range(3,2,6));
/// rng.numbers.push_back(Number::Range(-4,1,-2));
/// println!("{}", rng); // 1,3:2:6,-4:-2
/// println!("{:?}", rng.collect::<Vec<i64>>()); // [1, 3, 5, -4, -3, -2]
/// #     Ok(())
/// # }
/// ```
#[derive(Debug)]
pub enum Number<T> {
    Single(T),
    Range(T, T, T),
}

impl<T: num::Zero + std::cmp::PartialOrd + Copy> Number<T> {
    /// Checks the validity of the number/range
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use number_range::Number;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// assert!(Number::Single(1).is_valid());
    /// assert!(Number::Range(3,2,6).is_valid());
    /// assert!(Number::Range(-4,1,-2).is_valid());
    /// #     Ok(())
    /// # }
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            Number::Single(_) => true,
            Number::Range(start, step, end) => {
                ((start <= end) && (step > &num::Zero::zero()))
                    || ((start >= end) && (step < &num::Zero::zero()))
            }
        }
    }
    /// Opposite of is_valid
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use number_range::Number;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// assert!(Number::Range(3,-2,6).is_invalid());
    /// assert!(Number::Range(4,1,2).is_invalid());
    /// #     Ok(())
    /// # }
    /// ```
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

/// Options for the NumberRange
///
/// For example, if you're dealing with unsigned numbers then you can
/// use `-` as a range separator to parse ranges from many sources.
///
/// ```rust
/// # use std::error::Error;
/// # use number_range::NumberRangeOptions;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///     let rng: Vec<usize> = NumberRangeOptions::default()
///         .with_range_sep('-')
///         .parse("1,4,6-10,14")
///         .unwrap()
///         .collect();
///     println!("{:?}", rng);
/// #   Ok(())
/// # }
/// ```
///
/// Since it is made using generics to be able to pass as many types
/// of numeric types as possible, you might have to give the types in
/// between when rust cannot infer it.
///
/// It can be inferred if you have intermediate variables with type.
/// ```rust
/// # use std::error::Error;
/// # use number_range::{NumberRange,NumberRangeOptions};
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///     let rng: NumberRange<usize> = NumberRangeOptions::default()
///         .with_range_sep('-')
///         .parse("1,4,6-10,14")
///         .unwrap();
///     println!("{:?}", rng.collect::<Vec<usize>>());
/// #   Ok(())
/// # }
/// ```
///
/// ```rust
/// # use std::error::Error;
/// # use number_range::NumberRangeOptions;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// NumberRangeOptions::new()
///              .with_list_sep(',')
///              .with_range_sep('-')
///              .parse::<usize>("1,3-10,14")?;
/// #     Ok(())
/// # }
/// ```
///
/// All the numbers in the string must be of the same type that you
/// want to parse into, due to that restriction even the step needs to
/// be unsigned for unsigned number (meaning `"4:-1:1"` would fail
/// even if the final output should be unsigned).
#[derive(Debug)]
pub struct NumberRangeOptions {
    pub list_sep: char,
    pub range_sep: char,
}

/// NumberRange struct, once you've parsed the string you can iterate though it.
///
/// ```rust
/// # use std::error::Error;
/// # use number_range::NumberRange;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// NumberRange::<i64>::default()
///     .parse_str("-10,3:10,14:2:20")?;
/// #     Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct NumberRange<'a, T> {
    pub numbers: VecDeque<Number<T>>,
    original_repr: Option<&'a str>,
    pub options: NumberRangeOptions,
}

impl<'a, T: std::fmt::Display + num::One + std::cmp::PartialEq> std::fmt::Display
    for NumberRange<'a, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = self
            .numbers
            .iter()
            .map(|n| match n {
                Number::Single(v) => format!("{}", v),
                Number::Range(s, i, e) => {
                    if i.is_one() {
                        format!("{}{}{}", s, self.options.range_sep, e)
                    } else {
                        format!("{}{}{}{1}{}", s, self.options.range_sep, i, e)
                    }
                }
            })
            .join(&self.options.list_sep.to_string());
        write!(f, "{}", repr)
    }
}

impl<'a, T: Copy + std::ops::Add<Output = T> + std::cmp::PartialOrd + num::Zero> Iterator
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
                // checking this one coz people can insert their invalid ranges or parse invalid ones
                if self.numbers[0].is_valid() {
                    let next_step = Number::Range(start + step, step, end);
                    // checking here to always have valid steps
                    if next_step.is_valid() {
                        self.numbers[0] = next_step;
                    } else {
                        self.numbers.pop_front();
                    }
                    Some(start)
                } else {
                    self.numbers.pop_front();
                    self.next()
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
    /// New struct with default options
    pub fn new() -> Self {
        Self {
            list_sep: ',',
            range_sep: ':',
        }
    }

    /// Change the list separator character (default `,`)
    pub fn with_list_sep(mut self, sep: char) -> Self {
        self.list_sep = sep;
        self
    }

    /// Change the range separator character (default `:`)
    pub fn with_range_sep(mut self, sep: char) -> Self {
        self.range_sep = sep;
        self
    }

    /// Same as NumberRange::parse_str(), Makes a NumberRange and parses
    /// the string.
    pub fn parse<'a, T: std::str::FromStr + num::One + Copy>(
        self,
        numstr: &'a str,
    ) -> Result<NumberRange<T>, String> {
        let nr = NumberRange::from_options(self);
        nr.parse_str(numstr)
    }
}

impl<'a, T> Default for NumberRange<'a, T> {
    fn default() -> Self {
        Self {
            numbers: VecDeque::new(),
            original_repr: None,
            options: NumberRangeOptions::default(),
        }
    }
}

impl<'a, T: std::str::FromStr + num::One + Copy> NumberRange<'a, T> {
    /// New NumberRange struct from NumberRangeOptions
    pub fn from_options(options: NumberRangeOptions) -> Self {
        Self {
            numbers: VecDeque::new(),
            original_repr: None,
            options,
        }
    }

    /// Get the Original String that was used to parse the iterator
    pub fn original(&self) -> &str {
        self.original_repr.unwrap_or("")
    }

    /// Parse the human readable string (`numstr`).
    ///
    /// Once parsed the NumberRange struct can be used as an
    /// Iterator. Use `.collect::<T>()` to convert it into a vector.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use number_range::NumberRange;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// NumberRange::<i64>::default().parse_str("1,3,5:10")?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parse_str(mut self, numstr: &'a str) -> Result<Self, String> {
        self.original_repr = Some(numstr);
        self.parse()
    }
    pub fn parse(mut self) -> Result<Self, String> {
        if let Some(numstr) = self.original_repr {
            let numbers: VecDeque<Number<T>> = numstr
                .split(self.options.list_sep)
                .map(|seq_str| -> Result<Number<T>, String> {
                    match seq_str.matches(self.options.range_sep).count() {
                        0 => seq_str
                            .trim()
                            .parse::<T>()
                            .map_err(|_| "Not a Number".to_string())
                            .map(|v| Number::Single(v)),
                        1 => match seq_str.split_once(self.options.range_sep) {
                            Some((start, end)) => {
                                let start = start
                                    .trim()
                                    .parse::<T>()
                                    .map_err(|_| "Not a Number".to_string())?;
                                let end = end
                                    .trim()
                                    .parse::<T>()
                                    .map_err(|_| "Not a Number".to_string())?;
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
                                        .map_err(|_| "Not a Number".to_string())
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
            Ok(self)
        } else {
            Err("Nothing to Parse".to_string())
        }
    }
}

#[cfg(test)]
use rstest::rstest;

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    fn manual_build() {
        let mut rng = NumberRange::<i64>::default();
        rng.numbers.push_back(Number::Single(1));
        rng.numbers.push_back(Number::Range(3, 2, 6));
        rng.numbers.push_back(Number::Range(-4, 1, -2));
        assert_eq!(format!("{}", rng), "1,3:2:6,-4:-2");
        assert_eq!(rng.collect::<Vec<i64>>(), vec![1, 3, 5, -4, -3, -2]);
    }

    #[rstest]
    fn options_build() {
        let rng: NumberRange<usize> = NumberRangeOptions::default()
            .with_list_sep('*')
            .with_range_sep('/')
            .parse("1*3/5*9/2/15")
            .expect("Parsing should be succesful");
        assert_eq!(rng.collect::<Vec<usize>>(), vec![1, 3, 4, 5, 9, 11, 13, 15]);
    }

    #[rstest]
    fn manual_build_then_modify() {
        let mut rng = NumberRange::<i64>::default();
        rng.numbers.push_back(Number::Single(1));
        rng.numbers.push_back(Number::Range(3, 2, 6));
        rng.numbers.push_back(Number::Range(-4, 1, -2));
        let values = vec![1, 3, 5, -4, -3, -2];
        assert_eq!(format!("{}", rng), "1,3:2:6,-4:-2");
        for i in 0..4 {
            assert_eq!(rng.next().expect("Should have next"), values[i]);
        }
        assert_eq!(format!("{}", rng), "-3:-2");
        rng.numbers.push_back(Number::Single(1));
        assert_eq!(format!("{}", rng), "-3:-2,1");
        assert_eq!(rng.collect::<Vec<i64>>(), vec![-3, -2, 1]);
    }

    #[rstest]
    fn options_build_then_modify() {
        let mut rng: NumberRange<usize> = NumberRangeOptions::default()
            .with_list_sep(':')
            .with_range_sep('-')
            .parse("1:3-5:9")
            .expect("Parsing should be succesful");
        let values = vec![1, 3, 4, 5, 9];
        for i in 0..4 {
            assert_eq!(rng.next().expect("Should have next"), values[i]);
        }
        assert_eq!(format!("{}", rng), "9");
        rng.numbers.push_back(Number::Range(11, 2, 15));
        assert_eq!(format!("{}", rng), "9:11-2-15");
        assert_eq!(rng.collect::<Vec<usize>>(), vec![9, 11, 13, 15]);
    }

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
                .parse_str(numstr)
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
                .parse_str(numstr)
                .unwrap()
                .collect::<Vec<usize>>(),
            numvec
        );
    }

    #[rstest]
    #[case("200", vec!["200"])]
    #[case("1,4", vec!["1,4", "4"])]
    #[case("1:4", vec!["1:4", "2:4", "3:4", "4:4"])]
    #[case("10:-4:4", vec!["10:-4:4", "6:-4:4"])]
    #[case("4:-1:1", vec!["4:-1:1", "3:-1:1", "2:-1:1", "1:-1:1"])]
    #[case("1:4:10", vec!["1:4:10", "5:4:10", "9:4:10"])]
    fn format_test_loop(#[case] numstr: &str, #[case] numvec: Vec<&str>) {
        let mut rng: NumberRange<i64> = NumberRange::default().parse_str(numstr).unwrap();
        for fmt_str in numvec {
            assert_eq!(fmt_str, format!("{}", &rng));
            rng.next();
        }
        assert!(rng.next().is_none());
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
    #[case("4:-3:1", ':', vec![])]
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

    #[rstest]
    #[case("200", '-',vec![200])]
    #[case("1-4", '-', vec![1,2,3,4])]
    #[case("1:3:4", ':', vec![1, 4])]
    #[case("4:-3:1", ':', vec![4, 1])]
    #[case("-4:1", ':', vec![-4, -3, -2, -1, 0, 1])]
    #[case("1:-4", ':', vec![])]
    fn comma_test_range_i64(#[case] numstr: &str, #[case] sep: char, #[case] numvec: Vec<i64>) {
        assert_eq!(
            NumberRangeOptions::new()
                .with_range_sep(sep)
                .parse::<i64>(numstr)
                .unwrap()
                .collect::<Vec<i64>>(),
            numvec
        );
    }

    #[rstest]
    #[case("200", '-',vec![200.0])]
    #[case("1-4", '-', vec![1.0,2.0,3.0,4.0])]
    #[case("1:3:4", ':', vec![1.0, 4.0])]
    #[case("1:.5:4", ':', vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0])]
    #[case("4:-3:1", ':', vec![4.0, 1.0])]
    #[case("-4:1", ':', vec![-4.0, -3.0, -2.0, -1.0, 0.0, 1.0])]
    #[case("1:-4", ':', vec![])]
    fn comma_test_range_f64(#[case] numstr: &str, #[case] sep: char, #[case] numvec: Vec<f64>) {
        assert_eq!(
            NumberRangeOptions::new()
                .with_range_sep(sep)
                .parse::<f64>(numstr)
                .unwrap()
                .collect::<Vec<f64>>(),
            numvec
        );
    }
}

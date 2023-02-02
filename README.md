# Introduction
This is a simple library that converts the numbers range in human readable string to numeric type. For example: `"1-2"` to `[1,2]` or `"1,3:5"` to `[1,3,4,5]`.

There are two separators, `list_sep` (default `,`) and `range_sep` (default `:`), the string is first separated by the list separators, and then the individual part is considered a range, there are 3 types of ranges:

- `number` ⇒ Single number (e.g. `3`)
- `start:end` ⇒ Inclusive Range with step 1 (e.g. `1:10`)
- `start:step:end` ⇒ Inclusive Range with variable step (e.g. `1:2:10`)

# Uses
The simple use case is:
```rust
NumberRange::<i64>::default()
	.parse_str("-10,3:10,14:2:20")?;
```
It'll return you an iterator that you can use to iterate through those numbers. You can collect it in a vector with `.collect::Vec<T>()`. If you run out of the Iterator and want to iterate again, you can use `.parse()`.

All the numbers in the string must be of the same type that you want to parse into, due to that restriction even the step needs to be unsigned for unsigned integer (meaning `"4:-1:1"` would fail even if the final output should be unsigned).

The separators can be customized using the `NumberRangeOptions`. For example, if you're dealing with unsigned numbers then you can use `-` as a range separator to parse ranges from many sources.
```rust
NumberRangeOptions::new()
             .with_list_sep(',')
             .with_range_sep('-')
             .parse::<usize>("1,3-10,14")?;
```

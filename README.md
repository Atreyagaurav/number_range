# Introduction
This is a simple library that converts the numbers range in human readable string to numeric type and vice versa. For example: convert between `"1-2"` & `[1,2]` or `"1,3:5"` & `[1,3,4,5]`.

There are mainly two separators (for more types refer docs), `list_sep` (default `,`) and `range_sep` (default `:`), the string is first separated by the list separators, and then the individual part is considered a range, there are 3 types of ranges:

- `number` ⇒ Single number (e.g. `3`)
- `start:end` ⇒ Inclusive Range with step 1 (e.g. `1:10`)
- `start:step:end` ⇒ Inclusive Range with variable step (e.g. `1:2:10`)

# Disclaimers
This is unstable library, I'll be changing a few things that might break the compatibility with older versions till I can figure things out to make the parsing optimum. 

Even though the name is called Number Range, this is made to be used with Integers in mind, although the generics does work for float numbers, the results might not be as expected (which are the limitations of using float operations in rust).

# Uses
## NumberRange
The simple use case is:
```rust
NumberRange::<i64>::default()
	.parse_str("-10,3:10,14:2:20")?;
```
It'll return you an iterator that you can use to iterate through those numbers. You can collect it in a vector with `.collect::Vec<T>()`. If you run out of the Iterator and want to iterate again, you can use `.parse()`.

All the numbers in the string must be of the same type that you want to parse into, due to that restriction even the step needs to be unsigned for unsigned integer (meaning `"4:-1:1"` would fail even if the final output should be unsigned).

## NumberRangeOptions
The separators can be customized using the `NumberRangeOptions`. For example, if you're dealing with unsigned numbers then you can use `-` as a range separator to parse ranges from many sources.
```rust
NumberRangeOptions::new()
             .with_list_sep(',')
             .with_range_sep('-')
             .parse::<usize>("1,3-10,14")?;
```
Parsing numbers with localization.
```rust
let rng: Vec<usize> = NumberRangeOptions::new()
             .with_list_sep('/')
             .with_range_sep('-')
             .with_group_sep(',')
             .with_whitespace(true)
             .parse("1,200/1, 400, 230")?.collect();
assert_eq!(rng, vec![1200, 1400230]);
```

From Rust `List` or `Vec`:
```rust
assert_eq!(
    format!("{}", NumberRange::default()
           .from_vec(&[1,3,4,5,6,7,8,9,10,14], None)),
                     "1,3:10,14");
```

# Links
- Crate: <https://crates.io/crates/number_range>
- Documentation: <https://docs.rs/number_range/latest/number_range/>

# Introduction
This is a simple library that converts the numbers range in human readable string to numeric type. For example: `"1-2"` to `[1,2]` or `"1,3:5"` to `[1,3,4,5]`.

There are two separators, `list_sep` (default `,`) and `range_sep` (default `:`), the string is first separated by the list separators, and then the individual part is considered a range, there are 3 types of ranges:

- `number` ⇒ Single number (e.g. `3`)
- `start:end` ⇒ Inclusive Range with step 1 (e.g. `1:10`)
- `start:step:end` ⇒ Inclusive Range with variable step (e.g. `1:2:10`)



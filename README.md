# collect_exact

Allows zero-cost collection into exact-size arrays and tuples.

## Usage

```
use collect_exact::CollectExact;

let iter = [1, 2, 3].into_iter();
let result = iter.collect_exact::<[u32; 3]>();

assert_eq!(result, Ok([1, 2, 3]));
```

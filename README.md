# slice2d

Ergonomic array slice for 2d array manipulation.

## Example

```rust
let vector = vec![1, 2, 3, 4, 5, 6];
let slice = vector.get_slice2d(3, 2).unwrap();
assert_eq!(slice[2][0], 5);
assert_eq!(&slice[1], &[3, 4]);
```

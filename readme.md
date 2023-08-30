Arc obtains internal variability

```rust
let a = ArcCell::new(10);
unsafe {
    let ref_mut_a = ArcCell::get_mut_uncheck( &a);
    *ref_mut_a = 11;
}
assert_eq!(*a, 11);
```
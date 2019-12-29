# Permesan
A simple way to generate permutations from a rust collection.

# Getting Started
To use `Permesan`, simply create a new `Permesan` iterator from any collection that implements the `IntoIterator` trait.
Then you may use it as you would any other iterator.

```rust
let v = vec![1, 2, 3];
let mut perms = Permesan::new(v);

// perms.next() == vec![1, 2, 3]
// perms.next() == vec![2, 1, 3]
```

# Limitations
Currently, Permesan only works on collections that can be cloned.

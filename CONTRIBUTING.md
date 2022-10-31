# Contribution Guidelines

This library attempts to provide identical interfaces to the original
rust implementation. This includes struct and function names, import paths
..etc. Please keep this in mind when making contributions.


## Generics

As generics cannot be directly exposed to python as is, a concrete
type/function is exposed instead. The concrete type name is then added as a
suffix to the exposed function name, for example:

```rust
lizard_encode::<D: Digest>(data: &[u8; 16])
```

is exposed to python in concrete form for `Sha256`:

```python
lizard_encode_sha256(data: bytes)
```

The same rule applies to overloaded functions.


## Traits

All traits of some struct that are to be made available to python are exposed
over the same python class.

## Operators

To preserve the the whole "identical interfaces" goal, operators overloaded in
rust should also be overloaded in python.

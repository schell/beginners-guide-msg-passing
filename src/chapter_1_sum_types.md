# ➕ Sum Types

> What are they?

Sum types are quite literally a "sum" of types. The idea is that you take one or more types and
put them together in an either/or combination. A sum type is a type that is inhabited by the values
of an **enumeration** of other types.

```rust
enum Bool {
    True,
    False,
}

let no = Bool::False;
```

```rust
enum Bit {
    On,
    Off,
}

let on = Bit::On;
```

But why stop there?! We can nest the values of sum types:

```rust
# enum Bool {
#     True,
#     False,
# }
#
# enum Bit {
#     On,
#     Off,
# }
#
enum BoolOrBit {
    Bool(Bool),
    Bit(Bit),
}

let off = BoolOrBit::Bit(Bit::Off);
```

Now again, but with generics:

```rust
# enum Bool {
#     True,
#     False,
# }
#
# enum Bit {
#     On,
#     Off,
# }
#
enum Either<A, B> {
    Left(A),
    Right(B),
}

let off: Either<Bool, Bit> = Either::Right(Bit::Off);
```

In the above example you can really start to see why they are called "sum types".
The possible values in `Either<A, B>` are literally the sum of the values in `A` and
the values in `B`.

## ✖️ Product Types
The existance of sum types implies that there exists "product" types, where the possible
values in a type are a product of others. Indeed, Rust's struct and tuple types are
product types:

```rust
# enum Bool {
#     True,
#     False,
# }
#
# enum Bit {
#     On,
#     Off,
# }
#
struct BoolAndBit {
    boolean: Bool,
    bit: Bit,
}
```

```rust
# enum Bool {
#     True,
#     False,
# }
#
# enum Bit {
#     On,
#     Off,
# }
#
let false_off: (Bool, Bit) = (Bool::False, Bit::Off);
```

You can see here that a tuple of type `(A, B)` will have `A * B` value inhabitants.
So for our `(Bool, Bit)` type that would be `2 * 2`, or 4 different combinations.

## Combine them all
You can even use a short hand for enumerated structs:

```rust
enum Things {
    Cat {
        name: String,
        is_a_prince: bool,
    },
    Mantis {
        fly_heads_consumed: u32,
    },
    Bool(bool),
}
```

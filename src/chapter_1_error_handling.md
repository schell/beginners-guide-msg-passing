# ⚠️ Error Handling

Rust has a nice error handling story. Because of [`sum types`](./chapter_1_sum_types.md) we can encode our
errors in types. Additionally we can use [`traits`](./chapter_1_traits.md) to include special functionality
across all error types.

But before we get to that, let's talk about `panic!`.

## `panic!` at the disco 🪩 🕺

The `panic!` macro is a special "macro function" (we haven't talked about macros yet) that
terminates the program with a nice error message. It can also display a stack trace if your
program is compiled with debugging symbols.

Here are some examples:

```rust
panic!("This burrito is good, but it sure is filling!");
```

```rust
fn only_panic() {
    panic!("The Human Torch was denied a bank loan");
}
```

Panicking is a bit like throwing an exception. It's good if the error is unrecoverable and the
program should halt.


## Result

With the powers of `sum types` we can define an error type:

```rust
pub enum OkOrErr<T, E> {
    // Everything is ok
    Ok(T),
    // There was an error
    Err(E),
}
```

And now we can use this in our fallible functions:

```rust
# #[derive(Debug)]
# pub enum OkOrErr<T, E> {
#     // Everything is ok
#     Ok(T),
#     // There was an error
#     Err(E),
# }
pub fn divide(num: f32, denom: f32) -> OkOrErr<f32, String> {
    if denom == 0.0 {
        return OkOrErr::Err("cannot divide - denominator is zero!".to_string());
    }

    OkOrErr::Ok(num / denom)
}

println!("{:?}", divide(1.0, 2.0));
println!("{:?}", divide(1.0, 0.0));
```

### The ? Operator

You can imagine that in longer functions it might become a pain in your wrist to continually unwrap or match
on result values of `OkOrErr::Ok(...)` or `OkOrErr::Err(...)`. Indeed it is! That's why Rust has a shorthand
operator that means essentially "unwrap the value or short circuit and return the error" - namely the `?`
character. You may be familiar with this in another language like Swift:

```rust
# #[derive(Debug)]
# pub enum OkOrErr<T, E> {
#     // Everything is ok
#     Ok(T),
#     // There was an error
#     Err(E),
# }
# pub fn divide(num: f32, denom: f32) -> OkOrErr<f32, String> {
#     if denom == 0.0 {
#         return OkOrErr::Err("cannot divide - denominator is zero!".to_string());
#     }
#
#     OkOrErr::Ok(num / denom)
# }
pub fn divide_some_things() -> OkOrErr<(), String> {
    println!("{}", divide(1.0, 2.0)?);
    println!("{}", divide(1.0, 0.0)?);
    OkOrErr::Ok(())
}

if let OkOrErr::Err(e) = divide_some_things() {
    eprintln!("{}", e);
}
```

You'll see that attempting to run this produces the error:
> error[E0277]: the `?` operator can only be applied to values that implement `Try`

So I jumped the gun a bit there - we can't use `?` on arbitrary types, at least not without implementing `Try` for
those types first.

But `Try` is not yet stable (it's fine it's just only in nightly).

But luckily, Rust already provides an error type just like `OkOrErr` called `Result`, and `Result` implements `Try`,
so let's rewrite our functions to use the built-in `Result` type:

```rust
pub fn divide(num: f32, denom: f32) -> Result<f32, String> {
    if denom == 0.0 {
        // it's notable here that we don't need to prefix `Result::` onto `Err` - this is because
        // use std::result::Result::*; is implicit in all std rust code
        return Err("cannot divide - denominator is zero!".to_string());
    }

    Ok(num / denom)
}
pub fn divide_some_things() -> Result<(), String> {
    println!("{}", divide(1.0, 2.0)?);
    println!("{}", divide(1.0, 0.0)?);
    Ok(())
}

if let Err(e) = divide_some_things() {
    println!("{}", e);
}
```

#### Option
There are other types that implement `Try`. Most notably is `Option`, which is the Rust equivalent of
a nullable type:

```rust
pub enum Option<T> {
    Some(T),
    None,
}
```

Because `Option` implements `Try` we can use the `?` operator to clean up our functions that return `Option`s:

```rust
pub fn only_three(n: u8) -> Option<u8> {
    if n == 3 {
        Some(3)
    } else {
        None
    }
}

pub fn stringify_only_three(n: u8) -> Option<String> {
    let _:u8 = only_three(n)?;
    Some("3".to_string())
}
```

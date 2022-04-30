# 🏁 Pattern Matching

> What is it?

* destructuring
* access to data
* like equality but for the **shape** of values
* branching / control

> How do we do it?

## Let

When a type has a single constructor you can pattern match using the `let` keyword:

```rust
struct MyRecord {
    name: String,
    age: u8,
}

fn record_string(record: MyRecord) -> String {
    let MyRecord{ name, age } = record;
    format!("{} is {} years old", name, age)
}

let value = MyRecord {
    name: "Frodo Baggins".to_string(),
    age: 50,
};

println!("{}", record_string(value));
```

But this doesn't work when the type has more than one constructor because we
can't handle all the forms that a value might take:

```rust,compile_fail,editable
enum Hobbit {
    Frodo {
        has_ring: bool,
    },
    Samwise {
        has_potatoes: bool,
    },
    Other {
        name: String,
        likes: String,
    }
}

fn hobbit_string(hobbit: Hobbit) -> String {
    let Hobbit::Frodo{ has_ring } = hobbit;
    format!("Frodo Baggins {} the ring", if has_ring { "has" } else { "doesn't have" })
}

fn main() {
    println!("{}", hobbit_string(Hobbit::Samwise{ has_potatoes: true }));
}
```

Using the `match` keyword:

```rust,editable
# enum Hobbit {
#     Frodo {
#         has_ring: bool,
#     },
#     Samwise {
#         has_potatoes: bool,
#     },
#     Other {
#         name: String,
#         likes: String,
#     }
# }
#
fn hobbit_string(hobbit: Hobbit) -> String {
    match hobbit {
        Hobbit::Frodo{ has_ring } => format!(
            "Frodo Baggins {} the ring",
            if has_ring {
                "has"
            } else {
                "doesn't have"
            }
        ),
        Hobbit::Samwise{ has_potatoes } => format!(
            "Samwise Gamgee ... potatoes? {}",
            if has_potatoes {
                "mix em, mash em, throw em in a stew"
            } else {
                "no thanks"
            }
        ),
        Hobbit::Other{ name, likes } => format!("{} likes {}", name, likes),
    }
}

fn main() {
    println!("{}", hobbit_string(Hobbit::Samwise{ has_potatoes: true}));
}
```

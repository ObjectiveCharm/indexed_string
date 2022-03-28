__⚠️⚠️This crate is just a practice work with not taking grapheme cluster into consideration and will have undefined behaviour when try to convert from string has grapheme clusters__
__Sadly,I have no idea how to recognize boundary of grapheme clusters__
## Indexing UTF-8 String

Rust's stdlib does not support index and access string with vec-like subscript syntax.
Slicing string could also be "dangerous" for it will *panic* and crash if you achieve the middle of single utf-8 char.

 > "You should use ranges to create string slices with caution, because doing so can crash your program"  
    
So I try to make this small and simple library  to make things simpler and less tiring.

The library offers wrapper type `IndexedString` based on vec witch supports indexing, iterating and modifying string.
And it can convert from and to string and string slice simply and safely like vec.

## Installation

Simply add dependency to `cargo.toml`

```toml
[dependencies]
#... other
indexed_string = "0.1.0"
```

## Usage

```rust
/// import mod to scope
use indexed_string::indexed_string::IndexedString;

let str = "Stand with Ukraine";

/// convert &str to IndexedString
let indexed_string = IndexedString::from(str);

/// immutably access string
assert_eq!(indexed_string[1], "t");
/// mutably access string 
indexed_string[0] = "s";

/// Get modified new string
println!("{}", indexed_string.to_string());

```

## Just naive implementation and experimental product

As its description, it is just a __naive__ implementation(I am not familiar with rust)

1. To detect utf-8 char and variation selector I write too much nest if statements, which make the code hard to read. Maybe using macro instead my helps?
2. I am not familiar with bit operations which make code worse.
3. I am not familiar with lifetime, so I simply use Box<String> to encapsulate String every single character, which may make the methods of wrapper more ugly and inefficient?

But it is tested and works :)

## Contribute

Feel free to fork it and open pull request for refactoring, bug fix and so on.

## License
MIT
for detail please visit [the website](https://opensource.org/licenses/MIT)


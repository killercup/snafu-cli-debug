# Derive Debug implementation on Snafu Errors that make pretty output

## What

The challenge: Error handling in Rust.  
The approach: Embrace `Result`s!  
The problem: It's not easy to give add nice contexts to errors.  
The solution: [Snafu](https://docs.rs/snafu)!

The challenge: Printing errors in CLI apps.  
The approach: Embrace `fn main() -> Result<(), Error>`!  
The problem: It prints using `Debug`, i.e., like `println!("{:?}", error)`.  
The solution: This crate!

## Usage

See example.

## Output

```console
$ cargo run --example simple -q -- foo
Error: Can't read file `foo`
Info: caused by No such file or directory (os error 2)
```

```console
$ cargo run --example simple -q -- foo
Error: Can't read file `foo`
Info: caused by No such file or directory (os error 2)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

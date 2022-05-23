
Demo projects:

* exifoo
* stm
* aws-lambda-runtime
* https://sebnilsson.com/blog/from-csharp-to-rust-code-basics/


## Starting a new project

rustup (https://www.rust-lang.org/tools/install)

`cargo new` or `cargo init`


## Dependency management

* no binaries
* semantic versioning
* centralized package management (crates.io)
* local packages for forks and proprietary code
* vendored dependencies
* backward compatibility and breaking changes

## IDE support

* https://github.com/rust-lang/rust-analyzer/graphs/contributors

Linter is in the compiler
Great community effort
Warnings, errors are helpful
Formatting


## Compiler

 * Targets (`rustc --print target-list`, `rustup target add x86_64-unknown-linux-musl`, `cargo build --target x86_64-unknown-linux-musl`)
 * check/debug/release
 * output - native binary, as fast as C/C++

* Hermetic, reproducible builds
* A bug in rust is an app's feature.
* if it compiles and all errors were handled it's unlikely to fail
* why Rust cannot be in Linux kernel yet (memalloc panics in primitives)


## Reading a json file

Error, Option, Enum
Unwrapping, Type wrapping in Enums


## Freebies

Zero-cost generics (https://peterkos.me/rust-const-generics/, https://news.ycombinator.com/item?id=31238942)
Zero-cost async
Memory safety
No garbage collector (https://mattwarren.org/2014/06/18/measuring-the-impact-of-the-net-garbage-collector/)


## Comparison

#### linter

* built into the compiler
* helpful (uncomment `impl Display` without `use`)
* goes hand-in-hand with the formatter
* global reformat (`cargo fmt --all`)
* custom formatting rules (`.rustfmt.toml`)

#### `dynamic`

* dynamic-param-no-bounds.png
* lambda-runtime/src/lib.rs #82 (Box<dyn>), #67 (returns an fn)
* https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/types/using-type-dynamic

It can be unknown at design time, but if the compiler can resolve it to just one concrete type it's OK.

* panic::catch_unwind #133 - not everything can be unwound, uses special marker, segfault unlikely
* unsafe example

#### namespace conflict

* namespace-conflict.png

`using` in C# refers to namespaces. In Rust it can be any lelvel: crate, module, struct, pub var.

FA.PartnerAPI.Common has class `Constants` which is also defined locally and the local class is preferred.  
If both versions of `Constants` have the same method or property the dev will have no obvious way of knowing which one is used.

```
use anyhow::Error;
use std::error::Error;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
```

#### nulls

* never-sure-what-is-nullable.png
* C# is too flexible, too much legacy, hard to retrofit (https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/nullable-value-types)
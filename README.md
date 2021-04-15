# structopt-yaml

An default value loader from YAML for structopt. Derived from [structopt-toml](https://github.com/dalance/structopt-toml). It combinates with [structopt](https://github.com/TeXitoi/structopt).

[![Actions Status](https://github.com/mstump/structopt-yaml/workflows/Rust/badge.svg)](https://github.com/mstump/structopt-yaml/actions)
[![Crates.io](https://img.shields.io/crates/v/structopt-yaml.svg)](https://crates.io/crates/structopt-yaml)
[![Docs.rs](https://docs.rs/structopt-yaml/badge.svg)](https://docs.rs/structopt-yaml)
[![codecov](https://codecov.io/gh/mstump/structopt-yaml/branch/master/graph/badge.svg)](https://codecov.io/gh/mstump/structopt-yaml)

## Usage

This crate must be used with `serde`, `serde_derive`, `structopt`, and `serde_yaml` explicitly.

```Cargo.toml
[dependencies]
serde          = "1.0"
serde_derive   = "1.0"
serde_yaml     = "0.8.17"
structopt      = "0.3.11"
structopt-yaml = "0.4.5"
```

## Example

If `derive(Deserialize)`, `derive(StructOptYaml)` and `serde(default)` are added to the struct with `derive(StructOpt)`, some functions like `from_args_with_yaml` can be used.

```rust
use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_yaml::StructOptYaml;

#[derive(Debug, Deserialize, StructOpt, StructOptYaml)]
#[serde(default)]
struct Opt {
    #[structopt(default_value = "0", short = "a")] a: i32,
    #[structopt(default_value = "0", short = "b")] b: i32,
}

fn main() {
    let yaml_str = r#"
        a: 10
    "#;
    let opt = Opt::from_args_with_yaml(yaml_str).expect("yaml parse failed");
    println!("a:{}", opt.a);
    println!("b:{}", opt.b);
}
```

The execution result is below.

```console
$ ./example
a:10        // value from YAML string
b:0         // value from default_value of structopt

$ ./example -a 20
a:20        // value from command line argument
b:0
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

/*!
`structopt-yaml` is an default value loader from YAML for structopt.

## Examples

The following example show a quick example of `structopt-yaml`.

If `derive(Deserialize)`, `derive(StructOptYaml)` and `serde(default)` are added to the struct
with `derive(StructOpt)`, some functions like `from_args_with_yaml` can be used.

```
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

The execution result of the above example is below.

```ignore
$ ./example
a:10        // value from YAML string
b:0         // value from default_value of structopt

$ ./example -a 20
a:20        // value from command line argument
b:0
```
*/

extern crate clap as _clap;
extern crate failure;
extern crate serde as _serde;
extern crate structopt as _structopt;

#[allow(unused_imports)]
#[macro_use]
extern crate structopt_yaml_derive;

#[doc(hidden)]
pub use structopt_yaml_derive::*;

use std::ffi::OsString;

/// Re-export of clap
pub mod clap {
    pub use _clap::*;
}
/// Re-export of serde
pub mod serde {
    pub use _serde::*;
}
/// Re-export of structopt
pub mod structopt {
    pub use _structopt::*;
}

pub trait StructOptYaml {
    /// Merge the struct from YAML and the struct from args
    fn merge<'a>(from_yaml: Self, from_args: Self, args: &_clap::ArgMatches) -> Self
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>;

    /// Creates the struct from `clap::ArgMatches` with initial values from YAML.
    fn from_clap_with_yaml(yaml_str: &str, args: &_clap::ArgMatches) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::DeserializeOwned,
    {
        let from_args: Self = _structopt::StructOpt::from_clap(&args);
        let from_yaml: Self = serde_yaml::from_str(&yaml_str)?;
        Ok(Self::merge(from_yaml, from_args, &args))
    }

    /// Creates the struct from command line arguments with initial values from YAML.
    fn from_args_with_yaml<'a>(yaml_str: &'a str) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::DeserializeOwned,
    {
        let clap = Self::clap();
        let args = clap.get_matches();
        Self::from_clap_with_yaml(yaml_str, &args)
    }

    /// Creates the struct from iterator with initial values from YAML.
    fn from_iter_with_yaml<'a, I>(yaml_str: &'a str, iter: I) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::DeserializeOwned,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let clap = Self::clap();
        let args = clap.get_matches_from(iter);
        Self::from_clap_with_yaml(yaml_str, &args)
    }
}

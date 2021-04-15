use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_yaml::StructOptYaml;

#[derive(Debug, Deserialize, StructOpt, StructOptYaml)]
#[serde(default)]
struct Opt {
    #[structopt(default_value = "0", short = "a")]
    a: i32,
    #[structopt(default_value = "0", short = "b")]
    b: i32,
}

fn main() {
    let yaml_str = r#"
        a: 10
    "#;
    let opt = Opt::from_args_with_yaml(yaml_str).expect("yaml parse failed");
    println!("a:{}", opt.a);
    println!("b:{}", opt.b);
}

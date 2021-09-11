use cmd_lib::*;
use std::process;
use structopt::StructOpt;
/// Simple program to greet a person
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    force: bool,

    /// Get template
    #[structopt(short = "t", long = "template")]
    template: Option<String>,

    /// This option is positional
    repo_name: String,
}

fn main() {
    let args = Opt::from_args();
    println!("{:?}", args);
    run_cmd!(echo "sup!").unwrap();

    let output = match args.template {
        Some(template) => process::Command::new("npx")
            .arg("create-next-app")
            .arg("-e")
            .arg(template)
            .arg(&args.repo_name)
            .output()
            .expect("failed"),
        None => process::Command::new("npx")
            .arg("create-next-app")
            .arg(&args.repo_name)
            .output()
            .expect("failed"),
    };

    let uh = output.stdout;
}

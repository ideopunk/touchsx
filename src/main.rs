// use cmd_lib::*;
// use std::process::{Command, Stdio};
use structopt::StructOpt;
// use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::fs::File;
use regex::Regex;
use std::io::prelude::*;
use convert_case::{Case, Casing};

/// Simple program to greet a person
#[derive(StructOpt, Debug)]
#[structopt(about = "CNRBRNS")]
struct Cnr {
    #[structopt(subcommand)]
    cmd: Option<Cmd>
 
    
}

#[derive(StructOpt, Debug)]
enum Cmd {
    Touch {
        name: String
    },

    Create {
        #[structopt(short, long)]
        force: bool,
    
        /// Get template
        #[structopt(short = "t", long = "template")]
        template: Option<String>,
    
        /// Get repo name
        repo_name: String,
    
    }
}

fn touch(name: &str) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    let re = Regex::new(r"\w+\.tsx*").unwrap();
    if re.is_match(name) {
        let (pref, ext) = name.split_once(".").unwrap();
        if ext == "tsx" {
            let format_pref = pref.to_case(Case::Pascal);
            let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", format_pref);
            file.write_all(content.as_bytes())?;
        } else if ext == "ts" {
            let format_pref = pref.to_case(Case::Camel);
            let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", format_pref);
            file.write_all(content.as_bytes())?;
        } 
        
    }
    Ok(())
}

fn create(force: bool, template: Option<String>, repo_name: &str) -> std::io::Result<()> {
    Ok(())
}

fn main() {
    let args = Cnr::from_args();
    println!("{:?}", args);

   let result =  match args.cmd {
        Some(cmd) => {
            let result = match cmd {
                Cmd::Touch {name } => touch(&name),
                Cmd::Create {force,template,repo_name} => create(force, template, &repo_name)
            };
        },
        None => println!("Hi! You're running version 0.1.0!")

    };
    // if args.version {
    //     println!("0.1.0");
    //     return
    // }
 
    // match args.repo_name {
    //     Some(repo_name) => {
    //         match args.template {
    //             Some(template) => Command::new("npx")
    //                 .arg("create-next-app")
    //                 .arg("-e")
    //                 .arg(template)
    //                 .arg(repo_name)
    //                 .stdout(Stdio::inherit())
    //                 .stderr(Stdio::inherit())
    //                 .output()
    //                 .expect("failed"),
    //             None => Command::new("npx")
    //                 .arg("create-next-app")
    //                 .arg(repo_name)
    //                 .stdout(Stdio::inherit())
    //                 .stderr(Stdio::inherit())
    //                 .output()
    //                 .expect("failed"),
    //         };
    //     },
    //     None => println!("naw")
    // };

   

    // let uh = output.stdout;
}

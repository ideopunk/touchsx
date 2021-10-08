// use cmd_lib::*;
// use std::process::{Command, Stdio};
use structopt::StructOpt;
// use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::fs::File;
use regex::Regex;
use std::io::prelude::*;
use convert_case::{Case, Casing};

/// Create a .tsx file. 
#[derive(StructOpt, Debug)]
#[structopt(about = "CNRBRNS")]
struct TSX {
    names: Vec<String>
}

fn touch(name: &str) -> std::io::Result<()> {
    let re = Regex::new(r"\w+\.[jt]sx*").unwrap();

    if re.is_match(name) {
        let mut file = File::create(name)?;
        let (pref, ext) = name.split_once(".").unwrap();
        if ext == "tsx" || ext == "jsx" {
            let format_name_cased = &pref.to_case(Case::Pascal);
            let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", format_name_cased);
            file.write_all(content.as_bytes())?;
        } else if ext == "ts" || ext == "js" {
            let format_name_cased = &pref.to_case(Case::Camel);
            let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", format_name_cased);
            file.write_all(content.as_bytes())?;
        }
    } else {
        
        let mut file = if name.contains(".") {
            File::create(format!("{}", name))?
        } else {
            File::create(format!("{}.tsx", name))?
        };

        let format_name = name.to_case(Case::Pascal);
        let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", 
            &format_name.split_once(".").unwrap_or((&format_name, "")).0);
        file.write_all(content.as_bytes())?;
        
    }
    Ok(())
}


fn main() {
    let args = TSX::from_args();

    if args.names.len() == 0 {
        println!("Please include a file name.");
        return
    }

    for name in args.names.iter() {
        let res = touch(&name);
        match res {
            Err(e) => eprintln!("{}", e),
            _ => println!(":)", ),

        }
    }
}

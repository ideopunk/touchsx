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
    names: Vec<String>
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


fn main() {
    let args = Cnr::from_args();
    println!("{:?}", args);
    println!("{:?}", args.names);


    for name in args.names.iter() {
        let res = touch(&name);
    }

}

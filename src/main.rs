use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;
// use regex::Regex;
use convert_case::{Case, Casing};
#[cfg(test)]
use std::path::Path;

/// Create a Typescript file. 
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Javascript file")]
struct TSX {
    /// Name of file
    names: Vec<String>
}

fn touch(name: &str) -> std::io::Result<()> {

    // add a file extension if none exists
    let extended_name = if name.contains(".") {
        String::from(name)
    } else {
        format!("{}.tsx", name)
    };

    let clone_name = extended_name.clone();

    // create the file
    let mut file = File::create(extended_name)?;

    // snag the function name from the file name

    let (pref, ext) = clone_name.split_once(".").unwrap();

    let simple_name = if pref.contains("/") {
        pref.rsplitn(2, "/").collect::<Vec<&str>>()[0]
    } else {
        pref
    };


    let format_name_cased =  if ext == "tsx" || ext == "jsx" {
        simple_name.to_case(Case::Pascal)
    } else  {
        simple_name.to_case(Case::Camel)
    }; 

    // write out the boilerplate
    let content = format!("export default function {}() {{\n\treturn (\n\n\t)\n}}", format_name_cased);
    file.write_all(content.as_bytes())?;
 
    Ok(())
}

fn touches(names: Vec<String>) {
    for name in names.iter() {
        let res = touch(&name);
        match res {
            Err(e) => eprintln!("{}", e),
            _ => (),

        }
    }
}

fn main() {
    let args = TSX::from_args();

    if args.names.len() == 0 {
        println!("Please include a file name.\nTry 'touchsx --help' for more information.");
        return
    }

    touches(args.names);
}

#[cfg(test)]
pub fn touch_one_file_with_ext() {
    let _ = touch("fake.tsx");
    assert_eq!(true, Path::new("fake.tsx").exists())
}

#[cfg(test)]
pub fn touch_one_file_without_ext() {
    let _ = touch("fake");
    assert_eq!(true, Path::new("fake.tsx").exists())
}

#[cfg(test)]
pub fn touch_multiple_files() {
    let _ = touches(vec![String::from("gom"), String::from("jabbar")]);
    assert_eq!(true, Path::new("gom.tsx").exists());
    assert_eq!(true, Path::new("jabbar.tsx").exists())

}
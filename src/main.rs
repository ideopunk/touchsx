use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;
use regex::Regex;
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
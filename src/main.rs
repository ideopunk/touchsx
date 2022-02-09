use convert_case::{Case, Casing};
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// Create a Typescript file.
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Typescript file")]
struct TSX {
    /// Name of file
    names: Vec<PathBuf>,
}

pub fn touch(prename: &PathBuf) -> std::io::Result<()> {
    // don't let the ref to current directory confuse the program.
    // let not_relative_name = if name.starts_with("./") {
    //     let (_, rest) = name..split_at(2);
    //     rest
    // } else {
    //     name
    // };
    let mut name = prename.clone();
    // add a file extension if none exists
    let _ = match name.extension() {
        None => name.set_extension("tsx"),
        _ => true,
    };

    let clone_name = name.clone();

    // create the file
    let mut file = fs::File::create(clone_name)?;

    // snag the function name from the file name
    let stem = name.file_stem().unwrap();

    let format_name_cased =
        if name.extension().unwrap() == "tsx" || name.extension().unwrap() == "jsx" {
            stem.to_str().unwrap().to_case(Case::Pascal)
        } else {
            stem.to_str().unwrap().to_case(Case::Camel)
        };

    // write out the boilerplate
    let content = format!(
        "export default function {}() {{\n\treturn (\n\n\t)\n}}",
        format_name_cased
    );
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn touches(names: Vec<PathBuf>) {
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
        return;
    }

    touches(args.names);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn touch_one_file_with_ext() {
        let _ = touch(&PathBuf::from("fake.tsx"));
        assert_eq!(true, Path::new("fake.tsx").exists())
    }

    #[test]
    fn touch_one_file_without_ext() {
        let _ = touch(&PathBuf::from("fake"));
        assert_eq!(true, Path::new("fake.tsx").exists())
    }

    #[test]
    fn touch_multiple_files() {
        let _ = touches(vec![PathBuf::from("gom"), PathBuf::from("jabbar")]);
        assert_eq!(true, Path::new("gom.tsx").exists());
        assert_eq!(true, Path::new("jabbar.tsx").exists())
    }

    #[test]
    fn touch_file_in_subdirectory() {
        let _ = touch(&PathBuf::from("src/fake"));
        let contents = fs::read_to_string("src/fake.tsx").unwrap();
        println!("{}", contents);
        let proper_contents = contents.contains(" Fake");
        assert_eq!(true, proper_contents)
    }
}

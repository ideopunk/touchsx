use std::fs;
use std::io::prelude::*;
use structopt::StructOpt;
use convert_case::{Case, Casing};

/// Create a Typescript file. 
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Typescript file")]
struct TSX {
    /// Name of file
    names: Vec<String>
}

pub fn touch(name: &str) -> std::io::Result<()> {
    // add a file extension if none exists
    let extended_name = if name.contains(".") {
        String::from(name)
    } else {
        format!("{}.tsx", name)
    };

    let clone_name = extended_name.clone();

    // create the file
    let mut file = fs::File::create(extended_name)?;

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

pub fn touches(names: Vec<String>) {
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
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn touch_one_file_with_ext() {
        let _ = touch("fake.tsx");
        assert_eq!(true, Path::new("fake.tsx").exists())
    }

    #[test]
    fn touch_one_file_without_ext() {
        let _ = touch("fake");
        assert_eq!(true, Path::new("fake.tsx").exists())
    }

    #[test]
    fn touch_multiple_files() {
        let _ = touches(vec![String::from("gom"), String::from("jabbar")]);
        assert_eq!(true, Path::new("gom.tsx").exists());
        assert_eq!(true, Path::new("jabbar.tsx").exists())
    
    }

    #[test]
    fn touch_file_in_subdirectory() {
        let _ = touch("src/fake");
        let contents = fs::read_to_string("src/fake.tsx").unwrap();
        println!("{}", contents);
        let proper_contents = contents.contains(" Fake");
        assert_eq!(true, proper_contents)
    }
}

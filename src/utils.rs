use std::path::PathBuf;
use std::collections::HashMap;
use itertools::Itertools;
use glob::{glob_with,MatchOptions};
use std::fs::File;
use std::io::{Write, Error};

/// Get vec of file paths in a directory
pub fn get_files(root_path:PathBuf, pattern:&str, recursive:bool, case_sensitive:bool ) -> Vec<PathBuf>{ 
    if !root_path.exists(){ 
        panic!("path not found: {}", root_path.display());
    }

    // Construct Glob options and pattern
    let mut path = root_path.clone();
    let options = MatchOptions {
        case_sensitive,
        ..Default::default()
    };
    if recursive{
        path.push("**");
    }
    
    path.push(pattern);
    let str_path = path.to_str().expect("valid path pattern");

    // Collect files
    let mut result: Vec<PathBuf> = vec![];
    for file in glob_with(str_path, options).unwrap() {
        match file{
            Ok(p) => result.push(p),
            Err(e) => eprint!("Couldn't glob {e}")
        };
    }

    result

}

/// Print clusters
pub fn print_output(img_paths: &Vec<&str>, clusters:HashMap<usize, Vec<usize>>){
    println!("Results:");
    for cluster in clusters{
        println!("{}, {}", img_paths[cluster.0], cluster.1.iter().map(|x| img_paths[*x]).join(", "));
    }
}

/// Write clusters to file
pub fn write_output(out_path: &str, img_paths: &Vec<&str>, clusters:HashMap<usize, Vec<usize>>) -> Result<(), Error> {
    let mut output = File::create(out_path)?;
    for cluster in clusters{
        write!(output, "{}, {}\n", img_paths[cluster.0], cluster.1.iter().map(|x| img_paths[*x]).join(", "))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_files(){
        let test_images: Vec<&str> = vec![
            "tests/images/duct.png",
            "tests/images/bad_image.png",
            "tests/images/chess.png",
            "tests/images/danger.png",
            "tests/images/ductice.png",
            "tests/images/ductrust.png",
            ];
        let glob_files = get_files(PathBuf::from("tests/images"),"*.png", false, false, );
        assert_eq!(glob_files.len(), test_images.len());
        for glob_file in glob_files{
            let gf = glob_file.to_str().unwrap();
            assert!(test_images.contains(&gf));
        }
    }
}
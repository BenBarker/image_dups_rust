use std::path::PathBuf;
use glob::{glob_with,MatchOptions};

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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_files(){
        let test_images: Vec<&str> = vec![
            "tests/duct.png",
            "tests/chess.png",
            "tests/danger.png",
            "tests/ductice.png",
            "tests/ductrust.png",
            "tests/duplicates/ductA.png",
            "tests/duplicates/ductB.png",
            "tests/duplicates/ductC.png",];
        let glob_files = get_files(PathBuf::from("tests"),"*.png", true, false, );
        assert_eq!(glob_files.len(), test_images.len());
        for glob_file in glob_files{
            let gf = glob_file.to_str().unwrap();
            assert!(test_images.contains(&gf));
        }
    }

}
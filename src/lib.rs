use itertools::Itertools;
use std::collections::{HashMap,HashSet};
use rayon::prelude::*;
use image_hasher::{ImageHash, Hasher, HasherConfig,};
use image::ImageError;

pub mod utils;

/// Configure and return an image hasher configured for DCT mean hash
pub fn make_hasher(size:u32) -> Hasher{
    let config = HasherConfig::new();
    let config = config.hash_size(size, size);
    let config = config.hash_alg(image_hasher::HashAlg::Mean);
    let config = config.preproc_dct();
    config.to_hasher()
}

/// Given a image path and a hasher return an ImageHash Result
pub fn hash_img(img_path: &str, hasher: &Hasher)-> Result<ImageHash,ImageError>{ 
    let image1 = image::open(img_path)?;
    Ok(hasher.hash_image(&image1))
}

/// Given vector of paths return vector of hash Results
pub fn hash_img_list(img_list: Vec<&str>, hasher: &Hasher) -> Vec<Result<ImageHash,ImageError>> {
    img_list.into_par_iter()
        .map(|x|hash_img(x, &hasher))
        .collect()
}

/// Cluster a vector of image hashes based on a threshold. A "match" means that hash distance <= threshold.
/// Images only get clustered once. They will be ignored for future comparisons if already matched.
pub fn get_clusters(hash_list: &Vec<Result<ImageHash,ImageError>>, threshold: u32) -> HashMap<usize, Vec<usize>> {
    
    // Result is a hashmap of vectors where k: index of image and v: vector of matching indices
    let mut result: HashMap<usize, Vec<usize>> = HashMap::new();

    // Place to hold indices that tested positive already
    let mut used: HashSet<usize> = HashSet::new();

    for i in (0..hash_list.len()).combinations(2){
        
        // Skip anything that couldn't be hashed
        let Ok(hash1) = &hash_list[i[0]] else {continue;};
        let Ok(hash2) = &hash_list[i[1]] else {continue;};

        if used.contains(&i[1]){ continue };

        if hash1.dist(&hash2) <= threshold{
            let entry = result.entry(i[0]).or_insert(vec![]);
            entry.push(i[1]);
            used.insert(i[1]);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    // Given vector of paths return vector of hash Results.
    // This is a single threaded variant used only for testing.
    fn _hash_img_list_single(img_list: Vec<&str>, hasher: &Hasher) -> Vec<Result<ImageHash,ImageError>> {
        img_list.into_iter()
        .map(|x|hash_img(x,&hasher))
        .collect()
    }

    #[test]
    fn same_hash() {
        let img_path = "tests/duct.png";
        let hasher = make_hasher(16);
        let hash1 = hash_img(img_path, &hasher).unwrap();
        let hash2 = hash_img(img_path, &hasher).unwrap();
        assert_eq!(hash1,hash2);
    }

    #[test]
    fn same_hash_list(){
        let img_list = vec!["tests/duct.png";2];
        let hasher = make_hasher(16);
        let hashes = hash_img_list(img_list, &hasher);
        let hashes: Vec<ImageHash> = hashes.into_iter().flatten().collect(); //unwrap
        assert_eq!(hashes[0], hashes[1]);
    }

    #[test]
    /// Test that parallel hash produces stable (same order) results as single threaded hash
    fn parallel_hash_stable(){
        let test_images: Vec<&str> = vec![
            "tests/duct.png",
            "tests/chess.png",
            "tests/danger.png",
            "tests/ductice.png",
            "tests/ductrust.png"
            ];

        let hasher = make_hasher(16);
        let hashes_single = _hash_img_list_single(test_images.clone(), &hasher);
        let hashes_single: Vec<ImageHash> = hashes_single.into_iter().flatten().collect(); //unwrap
        
        let hashes_parallel = hash_img_list(test_images, &hasher);
        let hashes_parallel: Vec<ImageHash> = hashes_parallel.into_iter().flatten().collect(); //unwrap
        assert_eq!(hashes_single, hashes_parallel);
    }

    #[test]
    fn test_get_clusters(){
        let test_images: Vec<&str> = vec![
            "tests/duct.png",
            "tests/chess.png",
            "tests/danger.png",
            "test/bad_image.png",
            "tests/ductice.png",
            "tests/ductrust.png",
            "tests/duplicates/ductA.png",
            "tests/duplicates/ductB.png",
            "tests/duplicates/ductC.png",
            "tests/duplicates2/dangerA.png",
            "tests/duplicates2/dangerB.png",];

        let hasher = make_hasher(16);
        let hashes = hash_img_list(test_images.clone(), &hasher);
        
        // Exact matches
        let hashmap = get_clusters(&hashes, 0);
        assert_eq!(hashmap[&0], vec![6,7,8]);
        assert_eq!(hashmap[&2], vec![9,10]);

        //Pretty close matches
        let hashmap = get_clusters(&hashes, 8);
        assert_eq!(hashmap[&0], vec![4,5,6,7,8]);

    }


}

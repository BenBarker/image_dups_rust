use rayon::prelude::*;
use image_hasher::{ImageHash, Hasher, HasherConfig};

/// Configure and return a generic image hasher configured for DCT mean hash
pub fn make_hasher(size:u32) -> Hasher{
    let config = HasherConfig::new();
    let config = config.hash_size(size, size);
    let config = config.hash_alg(image_hasher::HashAlg::Mean);
    let config = config.preproc_dct();
    config.to_hasher()
}

/// Given a path and a hasher return an ImageHash Result
pub fn hash_img(img_path: &str, hasher: &Hasher)-> Result<ImageHash,image::ImageError>{ 
    let image1 = image::open(img_path)?;
    Ok(hasher.hash_image(&image1))
}

/// Given vector of paths return vector of hash Results
pub fn hash_img_list(img_list: Vec<&str>, hasher: &Hasher) -> Vec<Result<ImageHash,image::ImageError>> {
    img_list.into_par_iter()
        .map(|x|hash_img(x, &hasher))
        .collect()
}

// Given a list of hashes return vector of hashes with hamming dist < threshold
pub fn get_similar(hash_list: Vec<ImageHash>, threshold: usize) -> Vec<u32>{
    return vec![42];
    todo!("Do this");
    //let it = (0..2).cartesian_product("αβ".chars());
    //itertools::assert_equal(it, vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);
}


#[doc(hidden)]
/// Given vector of paths return vector of hash Results (single threaded, for testing)
fn _hash_img_list_single(img_list: Vec<&str>, hasher: &Hasher) -> Vec<Result<ImageHash,image::ImageError>> {
    img_list.into_iter()
    .map(|x|hash_img(x,&hasher))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn same_hash() {
        let img_path = "test/duct.png";
        let hasher = make_hasher(16);
        let hash1 = hash_img(img_path, &hasher).unwrap();
        let hash2 = hash_img(img_path, &hasher).unwrap();
        assert_eq!(hash1,hash2);
    }

    #[test]
    fn same_hash_list(){
        let img_list = vec!["test/duct.png";2];
        let hasher = make_hasher(16);
        let hashes = _hash_img_list_single(img_list, &hasher);
        let hashes: Vec<ImageHash> = hashes.into_iter().flatten().collect(); //unwrap
        assert_eq!(hashes[0], hashes[1]);
    }

    #[test]
    fn same_hash_list_parallel(){
        let img_list = vec!["test/duct.png";2];
        let hasher = make_hasher(16);
        let hashes = hash_img_list(img_list, &hasher);
        let hashes: Vec<ImageHash> = hashes.into_iter().flatten().collect(); //unwrap
        assert_eq!(hashes[0], hashes[1]);
    }

    #[test]
    fn parallel_hash_stable(){
        let test_images: Vec<&str> = vec!["test/duct.png","test/chess.png","test/danger.png","test/ductice.png","test/ductrust.png"];

        let hasher = make_hasher(16);
        let hashes_single = _hash_img_list_single(test_images.clone(), &hasher);
        let hashes_single: Vec<ImageHash> = hashes_single.into_iter().flatten().collect(); //unwrap
        
        let hashes_parallel = hash_img_list(test_images.clone(), &hasher);
        let hashes_parallel: Vec<ImageHash> = hashes_parallel.into_iter().flatten().collect(); //unwrap
        assert_eq!(hashes_single, hashes_parallel);
    }
}

use image_dups;
use image_hasher::ImageHash;

use std::time::Instant;

//TODO: ArgParse shit, better error handling when parsing images (get rid of unwraps etc)

fn main(){


    let before = Instant::now();

    let img_list = vec!["/home/ben/nurse.jpeg";10];
    let hasher = image_dups::make_hasher(32);
    let hashes = image_dups::hash_img_list(img_list, &hasher);
    let hashes: Vec<ImageHash> = hashes.into_iter().flatten().collect(); //unwrap results
    println!("Elapsed time multi: {:.2?}", before.elapsed());
}
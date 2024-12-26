//! Image duplicate finder
//! Each image is p-hashed and the hamming distance between each image pair is calculated
//! Images with a distance under a certain threshold are considered "duplicates"
//! Outputs a list of found duplicate "clusters".

use std::time::Instant;
use std::path::PathBuf;
use image_dups::{get_clusters, utils};
use clap::builder::TypedValueParser;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="Cluster images based on visual similarity.", long_about = None)]
pub struct Args{
    /// Path to image directory
    #[arg(short,long)]
    directory: PathBuf,

    ///Glob pattern used to gather image files
    #[arg(default_value="*.png",short,long)]
    pattern: String,

    ///Recurse into subdirectories
    #[arg(short,long)]
    recurse: bool,

    ///Hash size
    #[arg(
        long,
        short='s',
        default_value_t = 16,
        value_parser = clap::builder::PossibleValuesParser::new(["4","8","16","32","64","128","256","512"])
            .map(|s| s.parse::<u32>().unwrap()),
    )]
    hash_size: u32,

    ///Min Distance threshold. 0 = only identical images match, up to hash_size = everything matches
    #[arg(long,short,default_value_t=4)]
    min_distance: u32,

    ///Output file. if not specified then output is only printed.
    #[arg(short,long)]
    out_file: Option<String>,

}

fn main(){
    let args = Args::parse();

    let img_list = image_dups::utils::get_files(args.directory,
        &args.pattern, 
        args.recurse, 
        false);

    if img_list.len() < 1{
        panic!("No images found");
    }
    println!("Found {} images.", &img_list.len());
    let img_list = &img_list.iter().map(|x| x.to_str().unwrap()).collect();

    println!("Hashing images...");
    let hasher = image_dups::make_hasher(args.hash_size);
    let before = Instant::now();
    let hashes = image_dups::hash_img_list(img_list, &hasher);

    println!("Clustering...");
    let clustermap = get_clusters(&hashes, args.min_distance);

    match args.out_file {
        Some(out_path) => utils::write_output(out_path.as_str(), img_list, clustermap).expect("write failed"),
        _ => utils::print_output(img_list, clustermap),
    };
    println!("Elapsed time: {:.2?}", before.elapsed());


}
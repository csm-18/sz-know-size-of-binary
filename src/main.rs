use std::{env, fs};
use std::path::Path;

const VERSION:&str = "1.0.0";
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("sz {VERSION}");
        println!("A simple tool to know size of a binary.");
    }else if args.len() == 2 {
        let bin_path = Path::new(args[1].as_str());
        let bin_info = match fs::metadata(bin_path) {
            Ok(info) => info,
            Err(_) => {panic!("Some error occured!");},
        };
        if bin_info.is_file() {
            let file_size = bin_info.len();
            println!("name: {}", args[1]);
            println!("type: file");
            if file_size/1_000_000_000 > 0 {
                println!("Size in GBs: {:.3} GBs", (file_size as f64)/(1_000_000_000 as f64));
            }else if file_size/1_048_576 > 0 {
                println!("Size in MBs: {:.3} MBs", (file_size as f64)/(1_048_576 as f64));
            }else if file_size/1024 > 0 {
                println!("Size in KBs: {:.3} KBs",(file_size as f64)/(1024 as f64));
            }else {
                println!("Size in Bytes: {} bytes",file_size);
            }
        }else {
            println!("Only files are supported!");
        }
    }else {
        println!("Enter only one file name!");
    }
}

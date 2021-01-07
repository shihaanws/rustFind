use std::error::Error; 
use std::path::Path;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};




fn count_words(f: std::fs::File) {
    let mut words_map = HashMap::new();

    for line in BufReader::new(f).lines() {
        line.unwrap()
            .split_whitespace()
            .for_each(|word| *words_map.entry(word.to_owned()).or_insert(0) += 1);
            println!("Word Occurences")
    }

    for (key, val) in words_map {
        println!(" {} : {}", key, val);
    }
}

fn main()-> Result<(), Box<dyn Error>> {

    let folder_path ="./folderPath";
    let path = Path::new(folder_path);

    for file_result in path.read_dir()? {

        let file = file_result?;
        let file_path = file.path();

        println!("File Name : {}",file.file_name().to_string_lossy());

        // println!("File path : {}",file_path.display());
        
        let file = File::open(file_path).unwrap();

        count_words(file);


}
Ok(()) 

}





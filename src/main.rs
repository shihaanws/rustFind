use std::error::Error; 
use std::path::Path;
use std::fs;
use std::borrow::Cow;  //(copy on write)


fn counting_function(file_name:Cow<str>) -> std::io::Result<()> {

	let folder_path = "/home/shihaanws/Desktop/";
	let file_path = folder_path.to_string()+&file_name.to_string();
    let metadata = fs::metadata(file_path)?;
    let counter=metadata.len()-1;
    println!("The Text Count is {}",counter);
    
Ok(())
}



fn main()-> Result<(), Box<dyn Error>> {
    let folder_path ="/home/shihaanws/Desktop/";
    let path = Path::new(folder_path);
    for file_result in path.read_dir()? {
        let file = file_result?;
        println!("File Name : {}",file.file_name().to_string_lossy());
        counting_function(file.file_name().to_string_lossy()).expect("Unable to find");

}
Ok(()) 

}


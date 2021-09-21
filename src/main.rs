// https://www.awaresystems.be/imaging/tiff/tifftags/docs/alias.html
// Convert BGRA 3 bit frame from DeckLink to TIF formatted unsigned char buffer
// Tiff overview: http://www.fileformat.info/format/tiff/egff.htm
// Tiff spec : https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf
// https://www.awaresystems.be/imaging/tiff/tifftags/rowsperstrip.html
//
// The TIFF buffer composed here is organized into three sections.  The first section is the image file header (IFH).
// The second section is the image data.  The third section is the image file directory (IFD).
//
// The TIFF header is 8 bytes long.  The first 2 bytes define the byte order. (0x49 0x49 == little-endian)
// The second two bytes are magic numbers that define the buffer as a TIFF file.
// The last four bytes set the byte offset of the first image file directory (IFD).
//
// The data section includes the bytes used to describe the pixel color values.  The TIFF buffer generated
// here is uncompressed pixel data.
//
// The IFD includes the list of tags and the tag data.  In a TIFF file, tags are used to define atributes of the image.

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
// extern crate num_derive;

use skora;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path_string = args[1].clone();

    let mut paths = Vec::new();
    match path_string.ends_with(".tif") | path_string.ends_with(".tiff") {
        true => paths.push(path_string),
        false => {
            if Path::new(&path_string).is_dir() {
                for entry in fs::read_dir(Path::new(&path_string))? {
                    let dir_entry = entry?;
                    let path_as_string = dir_entry.path().to_string_lossy().to_string();
                    if path_as_string.ends_with(".tif") | path_as_string.ends_with(".tiff") {
                        paths.push(path_as_string);
                    }
                }
            }
        }
    }
    println!("path {:?}", paths);

    for file_path_string in paths {
        println!("");
        println!("Processing file {}", file_path_string);

        skora::convert_file(file_path_string)?;
    }

    Ok(())
}

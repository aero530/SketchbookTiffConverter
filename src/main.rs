//! Sketchbook Tiff to Open Raster Converter
//!
//! This is a command line program that will convert a single tiff file or directory of tiff files
//! that were created using Autodesk Sketchbook and convert them to Open Raster images (ora) so they
//! can be opened / modified using other software (such as Gimp or Krita).  Most importantly,
//! the layer information is preserved.
//!
//! Tiff files are used as a storage mechanism for Autodesk Sketchbook images.  Normally tiff files
//! do not include layer information (ie they are single layer) but they do allow somewhat arbitrary data
//! to be stored in them by including multiple IFDs (image file directory) in a single image file or by
//! including additional data in Tags (which are stored inside IFDs).  Sketchbook takes advantage of this
//! by storing a composite version of the image (all the layers merged) as the main image in the tiff file
//! and putting all the layers (and thumbnail) in different IFDs inside the IFD of the main composite image.
//! This way, any program can open the tiff file and get the correct image, but if it doesn't support Sketchbook's
//! specific way of manipulating tiffs for layers, then only the composite image shows up (ie the layers are lost).
//! As best I could find, there are no applications (other than Sketchbook) that support this tiff format.
//! While this isn't a 'normal' way to store layers, if what you are doing is documented its just as valid as
//! anything else.  There is limited documentation about this format (noteably none from Autodesk directly) but
//! <https://www.awaresystems.be/imaging/tiff/tifftags/docs/alias.html> does document the format allowing
//! for us to get all the layer information from the image as well.
//!
//!
//! ## USAGE ##
//! ```bash
//! > sketchbook-tiff-converter.exe [FLAGS] --path <path>
//! ```
//! ## FLAGS ##
//!
//! * -h, --help : Prints help information
//! * -l, --layer : Exports layers of each file processed as tiff files in addition to exporting the ora file (helpful if a file doesn't convert correctly for some reason). Note that these are the raw tiff layers as stored by Sketchbook so they are flipped vertically, in BGRA (instead of standard RGBA), & the rgb values are premultiplied by the alpha channel value.
//! * -V, --version : Prints version information
//! * -v : Sets the level of verbosity (v, vv, vvv).  Increasing the number of v's is helpful for debug if a file doesn't convert correctly.
//!
//! ## OPTIONS ##
//!
//! * -p, --path <path>  : Path of the tiff file or directory of tiff files to convert
//!

use std::error::Error;
use std::fs;
use std::path::Path;

use log::{info, trace, LevelFilter};

#[macro_use]
extern crate clap;
use clap::App;

use skora::convert_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the program input arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path_string = matches.value_of("path").unwrap().to_string();
    let verbose = matches.occurrences_of("verbose");
    let export_tiff = matches.is_present("layer");

    // Initialize the logging environment
    // If the specified `log_level` is not one of `off`, `error`, `warn`, `info`, `debug`, or `trace`.
    let log_level_str = match verbose {
        0 => "error",
        1 => "info",
        2 => "debug",
        3 => "trace",
        _ => "error",
    };

    let log_level = log_level_str
        .parse::<LevelFilter>()
        .expect("Unable to parse log level");
    env_logger::builder()
        .filter(Some("sketchbook_tiff_converter"), log_level)
        .filter(Some("sketchbook-tiff-converter"), log_level)
        .filter(Some("SketchbookTiffConverter"), log_level)
        .filter(Some("skora"), log_level)
        .init();

    // Process the input path
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

    trace!("Paths {:#?}", paths);

    for file_path_string in paths {
        info!("Processing file {}", file_path_string);
        convert_file(file_path_string, export_tiff)?;
    }

    Ok(())
}

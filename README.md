# SketchbookTiffConverter #

Convert Sketchbook Tiff Files to Open Raster Images and retain layer information.

This is a command line program that will convert a single tiff file or directory of tiff files that were created using Autodesk Sketchbook and convert them to Open Raster images (ora) so they can be opened / modified using other software (such as Gimp or Krita).  Most importantly, the layer information is preserved.

## Using the application ##

Download the latest release version of the application.  Extract the executable and run it from the command prompt.  The easiest way to use the application is to put it in the same folder as your tiff files and run from there as the location of the images is just `'.'`

```bash
> sketchbook-tiff-converter.exe -p .
```

Otherwise you can give the application the location of a single tiff or a directory (folder) with many tiffs to process.

```bash
> sketchbook-tiff-converter.exe -p images\test.tiff
```

```bash
> sketchbook-tiff-converter.exe -p images
```

### USAGE ###
  
```bash
> sketchbook-tiff-converter.exe [FLAGS] --path <path>
```

### FLAGS ###

```text
-h, --help       Prints help information

-l, --layer      Exports layers of each file processed as tiff files in addition to exporting the ora file (helpful if a file doesn't convert correctly for some reason). Note that these are the raw tiff layers as stored by Sketchbook so they are flipped vertically, in BGRA (instead of standard RGBA), & the rgb values are premultiplied by the alpha channel value.

-V, --version    Prints version information

-v               Sets the level of verbosity (v, vv, vvv).  Increasing the number of v's is helpful for debug if a file doesn't convert correctly.
```

### OPTIONS ###

```text
-p, --path <path>    Path of the tiff file or directory of tiff files to convert
```

## What's special about Sketchbook Tiffs? ##

Tiff files are used as a storage mechanism for Autodesk Sketchbook images.  Normally tiff files do not include layer information (ie they are single layer) but they do allow somewhat arbitrary data to be stored in them by including multiple IFDs (image file directory) in a single image file or by including additional data in Tags (which are stored inside IFDs).  Sketchbook takes advantage of this by storing a composite version of the image (all the layers merged) as the main image in the tiff file and putting all the layers (and thumbnail) in different IFDs inside the IFD of the main composite image.  This way, any program can open the tiff file and get the correct image, but if it doesn't support Sketchbook's specific way of manipulating tiffs for layers, then only the composite image shows up (ie the layers are lost).  As best I could find, there are no applications (other than Sketchbook) that support this tiff format.  While this isn't a 'normal' way to store layers, if what you are doing is documented its just as valid as anything else.  There is limited documentation about this format (noteably none from Autodesk directly) but https://www.awaresystems.be/imaging/tiff/tifftags/docs/alias.html does document the format allowing for us to get all the layer information from the image as well.

## Dev Setup ##

This application is written in Rust.  To install Rust, follow the instructions at [rustup.rs](https://rustup.rs). Then download the code from the repo and go to that directory.

### Running the application ###

```bash
> cargo run --release images/test3.tiff
```

--release tells the compilier to build in release mode which greatly improves the performance of the application.

### Compile for later use ###

```bash
> cargo build --release
```

The compiled application is put in the target/release folder.

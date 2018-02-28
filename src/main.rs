extern crate fitsio;
use fitsio::fitsfile::ImageDescription;
use std::fs::remove_file;



fn main() {

    let mut shape = vec![512,1024];//I want to generate a 512-row, 1024-col file
    shape.reverse();//if this line removed, the generated fits file will be 1024-row 512-col
    let img_desc = ImageDescription {
        data_type: fitsio::types::ImageType::LONG_IMG,
        dimensions: shape.as_slice(),
    };

    let fname="a.fits".to_string();
    remove_file(&fname);
    let mut fits_file = fitsio::FitsFile::create(fname)
            .with_custom_primary(&img_desc)
            .open().unwrap();

    let hdu = fits_file.current_hdu().unwrap();
    let mut data1 = Vec::<i32>::new();
    data1.resize(512*1024, 0);
    hdu.write_section(&mut fits_file, 0, data1.len(), &data1);
}

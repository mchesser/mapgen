#[allow(dead_code)];

use std::io;
use std::vec;
use extra2::color::Rgb;


/// Main bitmap structure
pub struct Bitmap {
    width:  uint,
    height: uint,
    pixels: ~[u8]
}

impl Bitmap {
    /// Set a pixel at x,y to a specified color
    /// # Arugments
    /// `x` - x coordinate
    /// `y` - y coordinate
    /// `color` - the color to set
    pub fn set_pixel(&mut self, x: uint, y: uint, color: Rgb) {
        // Calculate the byte offset for x
        let i = (self.height - y - 1) * (self.width * 3) + x * 3;

        // Note: Pixel order is (blue, green, red)
        self.pixels[i + 0] = color.b;
        self.pixels[i + 1] = color.g;
        self.pixels[i + 2] = color.r;
    }

    /// Write the stored data to a file with given filename
    /// # Arguments
    /// `filename` - the file name to save the file to
    pub fn write_to_file(&self, filename: &str) {
        static FILE_HEADER_SIZE:  u32 = 14;
        static BMP_INFO_SIZE:     u32 = 40;
        static TOTAL_HEADER_SIZE: u32 = FILE_HEADER_SIZE + BMP_INFO_SIZE;

        let image_size = (self.height * self.width*3 + self.height * (self.width % 4)) as u32;
        let file_size = image_size + TOTAL_HEADER_SIZE;

        // Bitmap file header
        let file_header: [u8, ..FILE_HEADER_SIZE] = [
            'B' as u8, 'M' as u8,
            file_size as u8, (file_size>>8) as u8, (file_size>>16) as u8, (file_size>>24) as u8,
            0, 0,
            0, 0,
            TOTAL_HEADER_SIZE as u8, 0, 0, 0
        ];
        // Bitmap information header
        let info_header: [u8, ..BMP_INFO_SIZE] = [
            BMP_INFO_SIZE as u8, 0, 0, 0,
            self.width as u8, (self.width>>8) as u8, (self.width>>16) as u8, (self.width>>24) as u8,
            self.height as u8, (self.height>>8) as u8, (self.height>>16) as u8, (self.height>>24) as u8,
            1, 0,
            24, 0,
            0, 0, 0, 0,
            image_size as u8, (image_size>>8) as u8, (image_size>>16) as u8, (image_size>>24) as u8,
            72, 0, 0, 0,
            72, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0
        ];

        // Set up the file writer
        let mut file = io::File::create(&Path::new(filename));

        // Write the bitmap headers to file
        file.write(file_header);
        file.write(info_header);

        // Write data to file
        file.write(self.pixels);
    }

    /// Create a new bitmap
    /// # Arguments
    /// `width` - the width of the bitmap
    /// `height` - the height of the bitmap
    /// # Return
    /// A new bitmap filled with black
    pub fn new(width: uint, height: uint) -> Bitmap {
        Bitmap {
            width:  width,
            height: height,
            pixels: vec::from_elem(height * (width * 3 + width % 4), 0u8)
        }
    }
}

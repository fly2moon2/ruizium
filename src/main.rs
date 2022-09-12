// image / imagebuffer
extern crate image;
extern crate num_complex;

// qrcode generator
use qrcode_generator::QrCodeEcc;



fn main() {


    // qr code generator
    // https://docs.rs/qrcode-generator/latest/qrcode_generator/
    // output to memory in png
    let result2: Vec<u8> = qrcode_generator::to_png_to_vec("Hello world!", QrCodeEcc::Low, 1024).unwrap();
    println!("{:?}", result2);
    // output to png file
    qrcode_generator::to_png_to_file("samsung in the world!", QrCodeEcc::Low, 1024, "assets/test_qr_generator_output.png").unwrap();


    // qr code scan
    // https://github.com/piderman314/bardecoder/blob/master/tests/image_tests.rs
    //let img = image::open("assets/version1_example.jpg").unwrap();
    //let img = image::open("assets/version1_example2.jpg").unwrap();
    //let img = image::open("assets/qrsample2.png").unwrap();
    // read the qr code generated from the code above
    let img = image::open("assets/test_qr_generator_output.png").unwrap();
    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }


    // image programmatically
    //https://github.com/image-rs/image/blob/master/examples/fractal.rs
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).0;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("assets/fractal.png").unwrap();
}

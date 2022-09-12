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
}

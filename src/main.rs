use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("dump.bin").unwrap();

    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();

    {
        image::save_buffer(
                "dump.bmp",
                &data,
                750,
                70,
                image::ColorType::Gray(8),
            ).unwrap();
    }

    {
        image::save_buffer(
                "dump.png",
                &data,
                750,
                70,
                image::ColorType::Gray(8),
            ).unwrap();
    }
}
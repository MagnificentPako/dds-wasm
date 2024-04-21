use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mipmap_count(file: Vec<u8>) -> Option<u32> {
    let dds = image_dds::ddsfile::Dds::read(file.as_slice()).unwrap();
    dds.header.mip_map_count
}

#[wasm_bindgen]
pub fn decode(file: Vec<u8>, mipmap: u32) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let dds = image_dds::ddsfile::Dds::read(file.as_slice()).unwrap();
    let img = image_dds::image_from_dds(&dds, mipmap).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image_dds::image::ImageFormat::WebP,
    )
    .unwrap();
    bytes
}

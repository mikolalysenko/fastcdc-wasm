use wasm_bindgen::prelude::*;
use fastcdc::*;

#[wasm_bindgen]
pub fn get_chunks(bytes:&mut [u8], min_size: u32, avg_size:u32, max_size:u32) -> Vec<u32> {
    let mut cuts = Vec::new();
    let chunker = FastCDC::new(&bytes, min_size as usize, avg_size as usize, max_size as usize);
    for entry in chunker {
        cuts.push(entry.offset as u32);
    }
    cuts.push(bytes.len() as u32);
    cuts
}
use wasm_bindgen::prelude::*;
use amm_sdk_netsblox::amm::storage::Storage;

fn translate(content: &[u8], storage: Storage) -> Result<String, JsError> {
    console_error_panic_hook::set_once();
    let composition = storage.load_data(content).map_err(|e| JsError::new(&format!("parse error: {e}")))?;
    amm_sdk_netsblox::translate(&composition).map_err(|e| JsError::new(&format!("translate error: {e:?}")))
}

#[wasm_bindgen]
pub fn translate_musicxml(content: &[u8]) -> Result<String, JsError> {
    translate(content, Storage::MusicXML)
}

#[wasm_bindgen]
pub fn translate_midi(content: &[u8]) -> Result<String, JsError> {
    translate(content, Storage::MIDI)
}

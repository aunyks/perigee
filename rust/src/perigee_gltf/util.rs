use gltf::accessor::Accessor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GltfAccessorViewError {
    #[error("could not get accessor for provided glTF buffer")]
    NoAccessorFound,
}

pub fn access_gltf_bytes<'a>(
    gltf_bytes: &'a Vec<u8>,
    accessor: &'a Accessor,
) -> Result<&'a [u8], GltfAccessorViewError> {
    if let Some(buffer_view) = accessor.view() {
        let view_start = buffer_view.offset() + accessor.offset();
        Ok(&gltf_bytes[view_start..view_start + buffer_view.length()])
    } else {
        Err(GltfAccessorViewError::NoAccessorFound)
    }
}

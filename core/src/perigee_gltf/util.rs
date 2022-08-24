use gltf::accessor::Accessor;

pub fn access_gltf_bytes<'a>(gltf_bytes: &'a Vec<u8>, accessor: &'a Accessor) -> &'a [u8] {
    let buffer_view = accessor
        .view()
        .expect("Could not get glTF buffer's accessor.");

    let view_start = buffer_view.offset() + accessor.offset();
    &gltf_bytes[view_start..view_start + buffer_view.length()]
}

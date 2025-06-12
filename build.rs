fn main() {
    embed_resource::compile("build/windows/rbxlx-to-rojo.rc", embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

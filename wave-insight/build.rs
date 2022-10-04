use static_files::resource_dir;
/// gen
fn main() -> std::io::Result<()> {
    resource_dir("../web/dist").build()
}

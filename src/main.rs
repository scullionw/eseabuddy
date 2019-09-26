use pypack::{
    rust_embed::{self, RustEmbed},
    RustEmbedExt,
};

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Python;

fn main() {
    Python::dump_and_exec();
}

#[macro_use]
extern crate pypack;

use pypack::{rust_embed, RustEmbed, RustEmbedExt};

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Python;

fn main() {
    Python::dump_and_exec();
}

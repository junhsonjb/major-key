use std::io::Error;
/*
    Build script, primarily used to generate Rust code
    from src/request.proto using prost-build.
*/

fn main() -> Result<(), Error> {
    prost_build::compile_protos(&["src/request.proto"], &["src/"])?;
    Ok(())
}

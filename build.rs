use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    prepare_object_pool();
}

fn prepare_object_pool() {
    let out_path = Path::new("src/library/evaluator/object_pool.rs");
    let mut file = File::create(out_path).unwrap();

    writeln!(file, "use crate::object::Object;").unwrap();
    writeln!(file, "pub const SMALL_INTS: [Object; 256] = [").unwrap();
    for i in 0..=255 {
        writeln!(file, "    Object::Int({}),", i).unwrap();
    }
    writeln!(file, "];").unwrap();
}

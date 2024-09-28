use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() - 1 < 2 {
        panic!("args: <path to reliquary lib dir> <path to data dir>");
    }

    let reliquary_path = Path::new(args[1].as_str());
    let data_path = Path::new(args[2].as_str());

    protos(reliquary_path, data_path);
    packet_id(reliquary_path, data_path);
}

fn protos(reliquary_path: &Path, data_path: &Path) {
    let proto_dir = data_path.join("proto");

    println!("scanning {}", proto_dir.display());

    let protos: Vec<PathBuf> = proto_dir
        .read_dir()
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .ends_with(".proto")
        })
        .collect();

    for proto in protos.iter() {
        println!("detected proto: {}", proto.display());
    }

    println!("generating protos");

    let out_dir = reliquary_path.join("src/network/gen/proto");

    protobuf_codegen::Codegen::new()
        .pure()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .include(proto_dir)
        // Inputs must reside in some of include paths.
        .inputs(protos)
        .out_dir(&out_dir)
        .run()
        .unwrap();
}

fn packet_id(reliquary_path: &Path, data_path: &Path) {
    let json_path = data_path.join("packetIds.json");

    println!("reading packet ids");
    let json = File::open(json_path).expect("to read file");
    let map: Value = serde_json::from_reader(json).unwrap();
    let map = map.as_object().unwrap();

    let key_values: Vec<(&str, &str)> = map
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str().unwrap()))
        .collect();

    // TODO: use quote crate

    let mut output = "
// @generated\n"
        .to_string();

    key_values
        .iter()
        .map(|(id, s)| format!("pub const {s}: u16 = {id};\n"))
        .for_each(|s| output.push_str(s.as_str()));

    output.push_str(
        r#"

pub fn command_id_to_str(id: u16) -> Option<&'static str> {
    match id {
"#,
    );

    key_values
        .iter()
        .map(|(_, s)| {
            format!(
                r#"        {s} => Some("{s}"),
"#
            )
        })
        .for_each(|s| output.push_str(s.as_str()));

    output.push_str(
        r#"
        _ => None
    }
}"#,
    );

    let output_path = reliquary_path.join("src/network/gen/command_id.rs");
    std::fs::write(output_path, output).expect("to write command ids");
}

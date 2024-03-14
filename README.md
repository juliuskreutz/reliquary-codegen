# reliquary_codegen

need to update types for [reliquary](https://github.com/IceDynamix/reliquary)?
- create `data` directory with following structure
  - protos -> `./data/proto/*.proto`
  - packetIds.json -> `./data/packetIds.json`
- `cargo run -- <path to reliquary lib dir> <path to data dir>`
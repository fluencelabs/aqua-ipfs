pub const IPFS_EFFECTOR: &'static [u8] = include_bytes!("../ipfs-service/ipfs_effector.wasm");
pub const IPFS_FACADE: &'static [u8] = include_bytes!("../ipfs-service/ipfs_pure.wasm");
pub const CONFIG: &'static [u8] = include_bytes!("../ipfs-service/Config.toml");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub use build_info::PKG_VERSION as VERSION;

pub fn modules() -> std::collections::HashMap<&'static str, &'static [u8]> {
    maplit::hashmap! {
        "facade" => IPFS_FACADE,
        "effector" => IPFS_EFFECTOR,
    }
}

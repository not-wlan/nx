use crate::{ipc::cmif::sf, result::*};

pub trait IRandomInterface {
    ipc_cmif_interface_define_command!(generate_random_bytes: (out_buf: sf::OutMapAliasBuffer) => ());
}

use crate::result::*;
// use crate::ipc::cmif::sf;

pub trait IInformationInterface {
    ipc_cmif_interface_define_command!(get_program_id: (process_id: u64) => (program_id: u64));
}

pub trait IDebugMonitorInterface {
    ipc_cmif_interface_define_command!(get_application_process_id: () => (process_id: u64));
}

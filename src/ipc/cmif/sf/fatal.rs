use crate::{ipc::cmif::sf, result::*};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Policy {
    ErrorReportAndErrorScreen,
    ErrorReport,
    ErrorScreen,
}

pub trait IService {
    ipc_cmif_interface_define_command!(throw_with_policy: (rc: ResultCode, policy: Policy, process_id: sf::ProcessId) => ());
}

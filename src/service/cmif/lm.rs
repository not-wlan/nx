use crate::{ipc::cmif::sf, mem, result::*, service};

pub use crate::ipc::cmif::sf::lm::*;

pub struct Logger {
    session: sf::Session,
}

impl sf::IObject for Logger {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![
            ipc_cmif_interface_make_command_meta!(log: 0),
            ipc_cmif_interface_make_command_meta!(set_destination: 1),
        ]
    }
}

impl service::cmif::IClientObject for Logger {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl ILogger for Logger {
    fn log(&mut self, log_buf: sf::InAutoSelectBuffer) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 0] (log_buf) => ())
    }

    fn set_destination(&mut self, log_destination: LogDestination) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 1] (log_destination) => ())
    }
}

pub struct LogService {
    session: sf::Session,
}

impl sf::IObject for LogService {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![ipc_cmif_interface_make_command_meta!(open_logger: 0)]
    }
}

impl service::cmif::IClientObject for LogService {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl ILogService for LogService {
    fn open_logger(&mut self, process_id: sf::ProcessId) -> Result<mem::Shared<dyn sf::IObject>> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 0] (process_id) => (logger: mem::Shared<Logger>))
    }
}

impl service::cmif::IService for LogService {
    fn get_name() -> &'static str {
        nul!("lm")
    }

    fn as_domain() -> bool {
        false
    }

    fn post_initialize(&mut self) -> Result<()> {
        Ok(())
    }
}

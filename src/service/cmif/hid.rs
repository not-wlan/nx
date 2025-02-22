use crate::{ipc::cmif::sf, mem, result::*, service};

pub use crate::ipc::cmif::sf::hid::*;

pub struct AppletResource {
    session: sf::Session,
}

impl sf::IObject for AppletResource {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![ipc_cmif_interface_make_command_meta!(get_shared_memory_handle: 1)]
    }
}

impl service::cmif::IClientObject for AppletResource {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IAppletResource for AppletResource {
    fn get_shared_memory_handle(&mut self) -> Result<sf::CopyHandle> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 0] () => (shmem_handle: sf::CopyHandle))
    }
}

pub struct HidServer {
    session: sf::Session,
}

impl sf::IObject for HidServer {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![
            ipc_cmif_interface_make_command_meta!(create_applet_resource: 0),
            ipc_cmif_interface_make_command_meta!(set_supported_npad_style_set: 100),
            ipc_cmif_interface_make_command_meta!(set_supported_npad_id_type: 102),
            ipc_cmif_interface_make_command_meta!(activate_npad: 103),
            ipc_cmif_interface_make_command_meta!(deactivate_npad: 104),
            ipc_cmif_interface_make_command_meta!(set_npad_joy_assignment_mode_single: 123),
            ipc_cmif_interface_make_command_meta!(set_npad_joy_assignment_mode_dual: 124),
        ]
    }
}

impl service::cmif::IClientObject for HidServer {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IHidServer for HidServer {
    fn create_applet_resource(
        &mut self,
        aruid: sf::ProcessId,
    ) -> Result<mem::Shared<dyn sf::IObject>> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 0] (aruid) => (applet_resource: mem::Shared<AppletResource>))
    }

    fn set_supported_npad_style_set(
        &mut self,
        aruid: sf::ProcessId,
        npad_style_tag: NpadStyleTag,
    ) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 100] (npad_style_tag, aruid) => ())
    }

    fn set_supported_npad_id_type(
        &mut self,
        aruid: sf::ProcessId,
        controllers: sf::InPointerBuffer,
    ) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 102] (aruid, controllers) => ())
    }

    fn activate_npad(&mut self, aruid: sf::ProcessId) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 103] (aruid) => ())
    }

    fn deactivate_npad(&mut self, aruid: sf::ProcessId) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 104] (aruid) => ())
    }

    fn set_npad_joy_assignment_mode_single(
        &mut self,
        aruid: sf::ProcessId,
        controller: ControllerId,
        joy_type: NpadJoyDeviceType,
    ) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 123] (controller, aruid, joy_type) => ())
    }

    fn set_npad_joy_assignment_mode_dual(
        &mut self,
        aruid: sf::ProcessId,
        controller: ControllerId,
    ) -> Result<()> {
        ipc_cmif_client_send_request_command!([self.session.object_info; 124] (controller, aruid) => ())
    }
}

impl service::cmif::IService for HidServer {
    fn get_name() -> &'static str {
        nul!("hid")
    }

    fn as_domain() -> bool {
        false
    }

    fn post_initialize(&mut self) -> Result<()> {
        Ok(())
    }
}

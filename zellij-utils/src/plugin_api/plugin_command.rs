pub use super::generated_api::api::{
    event::EventNameList as ProtobufEventNameList,
    plugin_command::{
        plugin_command::Payload, CommandName, ExecCmdPayload, IdAndNewName, MovePayload,
        OpenCommandPanePayload, OpenFilePayload, PaneIdAndShouldFloat,
        PluginCommand as ProtobufPluginCommand, PluginMessagePayload,
        RequestPluginPermissionPayload, ResizePayload, SetTimeoutPayload, SubscribePayload,
        SwitchTabToPayload, SwitchToModePayload, UnsubscribePayload, SwitchSessionPayload
    },
    plugin_permission::PermissionType as ProtobufPermissionType,
    resize::ResizeAction as ProtobufResizeAction,
};

use crate::data::{PermissionType, PluginCommand, ConnectToSession};

use std::convert::TryFrom;

impl TryFrom<ProtobufPluginCommand> for PluginCommand {
    type Error = &'static str;
    fn try_from(protobuf_plugin_command: ProtobufPluginCommand) -> Result<Self, &'static str> {
        match CommandName::from_i32(protobuf_plugin_command.name) {
            Some(CommandName::Subscribe) => match protobuf_plugin_command.payload {
                Some(Payload::SubscribePayload(subscribe_payload)) => {
                    let protobuf_event_list = subscribe_payload.subscriptions;
                    match protobuf_event_list {
                        Some(protobuf_event_list) => {
                            Ok(PluginCommand::Subscribe(protobuf_event_list.try_into()?))
                        },
                        None => Err("malformed subscription event"),
                    }
                },
                _ => Err("Mismatched payload for Subscribe"),
            },
            Some(CommandName::Unsubscribe) => match protobuf_plugin_command.payload {
                Some(Payload::UnsubscribePayload(unsubscribe_payload)) => {
                    let protobuf_event_list = unsubscribe_payload.subscriptions;
                    match protobuf_event_list {
                        Some(protobuf_event_list) => {
                            Ok(PluginCommand::Unsubscribe(protobuf_event_list.try_into()?))
                        },
                        None => Err("malformed unsubscription event"),
                    }
                },
                _ => Err("Mismatched payload for Unsubscribe"),
            },
            Some(CommandName::SetSelectable) => match protobuf_plugin_command.payload {
                Some(Payload::SetSelectablePayload(should_be_selectable)) => {
                    Ok(PluginCommand::SetSelectable(should_be_selectable))
                },
                _ => Err("Mismatched payload for SetSelectable"),
            },
            Some(CommandName::GetPluginIds) => {
                if protobuf_plugin_command.payload.is_some() {
                    Err("GetPluginIds should not have a payload")
                } else {
                    Ok(PluginCommand::GetPluginIds)
                }
            },
            Some(CommandName::GetZellijVersion) => {
                if protobuf_plugin_command.payload.is_some() {
                    Err("GetZellijVersion should not have a payload")
                } else {
                    Ok(PluginCommand::GetZellijVersion)
                }
            },
            Some(CommandName::OpenFile) => match protobuf_plugin_command.payload {
                Some(Payload::OpenFilePayload(file_to_open_payload)) => {
                    match file_to_open_payload.file_to_open {
                        Some(file_to_open) => Ok(PluginCommand::OpenFile(file_to_open.try_into()?)),
                        None => Err("Malformed open file payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenFile"),
            },
            Some(CommandName::OpenFileFloating) => match protobuf_plugin_command.payload {
                Some(Payload::OpenFileFloatingPayload(file_to_open_payload)) => {
                    match file_to_open_payload.file_to_open {
                        Some(file_to_open) => {
                            Ok(PluginCommand::OpenFileFloating(file_to_open.try_into()?))
                        },
                        None => Err("Malformed open file payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenFile"),
            },
            Some(CommandName::OpenTerminal) => match protobuf_plugin_command.payload {
                Some(Payload::OpenTerminalPayload(file_to_open_payload)) => {
                    match file_to_open_payload.file_to_open {
                        Some(file_to_open) => {
                            Ok(PluginCommand::OpenTerminal(file_to_open.try_into()?))
                        },
                        None => Err("Malformed open terminal payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenTerminal"),
            },
            Some(CommandName::OpenTerminalFloating) => match protobuf_plugin_command.payload {
                Some(Payload::OpenTerminalFloatingPayload(file_to_open_payload)) => {
                    match file_to_open_payload.file_to_open {
                        Some(file_to_open) => Ok(PluginCommand::OpenTerminalFloating(
                            file_to_open.try_into()?,
                        )),
                        None => Err("Malformed open terminal floating payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenTerminalFloating"),
            },
            Some(CommandName::OpenCommandPane) => match protobuf_plugin_command.payload {
                Some(Payload::OpenCommandPanePayload(command_to_run_payload)) => {
                    match command_to_run_payload.command_to_run {
                        Some(command_to_run) => {
                            Ok(PluginCommand::OpenCommandPane(command_to_run.try_into()?))
                        },
                        None => Err("Malformed open open command pane payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenCommandPane"),
            },
            Some(CommandName::OpenCommandPaneFloating) => match protobuf_plugin_command.payload {
                Some(Payload::OpenCommandPaneFloatingPayload(command_to_run_payload)) => {
                    match command_to_run_payload.command_to_run {
                        Some(command_to_run) => Ok(PluginCommand::OpenCommandPaneFloating(
                            command_to_run.try_into()?,
                        )),
                        None => Err("Malformed open command pane floating payload"),
                    }
                },
                _ => Err("Mismatched payload for OpenCommandPaneFloating"),
            },
            Some(CommandName::SwitchTabTo) => match protobuf_plugin_command.payload {
                Some(Payload::SwitchTabToPayload(switch_to_tab_payload)) => Ok(
                    PluginCommand::SwitchTabTo(switch_to_tab_payload.tab_index as u32),
                ),
                _ => Err("Mismatched payload for SwitchToTab"),
            },
            Some(CommandName::SetTimeout) => match protobuf_plugin_command.payload {
                Some(Payload::SetTimeoutPayload(set_timeout_payload)) => {
                    Ok(PluginCommand::SetTimeout(set_timeout_payload.seconds))
                },
                _ => Err("Mismatched payload for SetTimeout"),
            },
            Some(CommandName::ExecCmd) => match protobuf_plugin_command.payload {
                Some(Payload::ExecCmdPayload(exec_cmd_payload)) => {
                    Ok(PluginCommand::ExecCmd(exec_cmd_payload.command_line))
                },
                _ => Err("Mismatched payload for ExecCmd"),
            },
            Some(CommandName::PostMessageTo) => match protobuf_plugin_command.payload {
                Some(Payload::PostMessageToPayload(post_message_to_payload)) => {
                    match post_message_to_payload.message {
                        Some(message) => Ok(PluginCommand::PostMessageTo(message.try_into()?)),
                        None => Err("Malformed post message to payload"),
                    }
                },
                _ => Err("Mismatched payload for PostMessageTo"),
            },
            Some(CommandName::PostMessageToPlugin) => match protobuf_plugin_command.payload {
                Some(Payload::PostMessageToPluginPayload(post_message_to_payload)) => {
                    match post_message_to_payload.message {
                        Some(message) => {
                            Ok(PluginCommand::PostMessageToPlugin(message.try_into()?))
                        },
                        None => Err("Malformed post message to plugin payload"),
                    }
                },
                _ => Err("Mismatched payload for PostMessageToPlugin"),
            },
            Some(CommandName::HideSelf) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("HideSelf should not have a payload");
                }
                Ok(PluginCommand::HideSelf)
            },
            Some(CommandName::ShowSelf) => match protobuf_plugin_command.payload {
                Some(Payload::ShowSelfPayload(should_float_if_hidden)) => {
                    Ok(PluginCommand::ShowSelf(should_float_if_hidden))
                },
                _ => Err("Mismatched payload for ShowSelf"),
            },
            Some(CommandName::SwitchToMode) => match protobuf_plugin_command.payload {
                Some(Payload::SwitchToModePayload(switch_to_mode_payload)) => {
                    match switch_to_mode_payload.input_mode {
                        Some(input_mode) => Ok(PluginCommand::SwitchToMode(input_mode.try_into()?)),
                        None => Err("Malformed switch to mode payload"),
                    }
                },
                _ => Err("Mismatched payload for SwitchToMode"),
            },
            Some(CommandName::NewTabsWithLayout) => match protobuf_plugin_command.payload {
                Some(Payload::NewTabsWithLayoutPayload(raw_layout)) => {
                    Ok(PluginCommand::NewTabsWithLayout(raw_layout))
                },
                _ => Err("Mismatched payload for NewTabsWithLayout"),
            },
            Some(CommandName::NewTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("NewTab should not have a payload");
                }
                Ok(PluginCommand::NewTab)
            },
            Some(CommandName::GoToNextTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("GoToNextTab should not have a payload");
                }
                Ok(PluginCommand::GoToNextTab)
            },
            Some(CommandName::GoToPreviousTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("GoToPreviousTab should not have a payload");
                }
                Ok(PluginCommand::GoToPreviousTab)
            },
            Some(CommandName::Resize) => match protobuf_plugin_command.payload {
                Some(Payload::ResizePayload(resize_payload)) => match resize_payload.resize {
                    Some(resize) => Ok(PluginCommand::Resize(resize.try_into()?)),
                    None => Err("Malformed switch resize payload"),
                },
                _ => Err("Mismatched payload for Resize"),
            },
            Some(CommandName::ResizeWithDirection) => match protobuf_plugin_command.payload {
                Some(Payload::ResizeWithDirectionPayload(resize_with_direction_payload)) => {
                    match resize_with_direction_payload.resize {
                        Some(resize) => Ok(PluginCommand::ResizeWithDirection(resize.try_into()?)),
                        None => Err("Malformed switch resize payload"),
                    }
                },
                _ => Err("Mismatched payload for Resize"),
            },
            Some(CommandName::FocusNextPane) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("FocusNextPane should not have a payload");
                }
                Ok(PluginCommand::FocusNextPane)
            },
            Some(CommandName::FocusPreviousPane) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("FocusPreviousPane should not have a payload");
                }
                Ok(PluginCommand::FocusPreviousPane)
            },
            Some(CommandName::MoveFocus) => match protobuf_plugin_command.payload {
                Some(Payload::MoveFocusPayload(move_payload)) => match move_payload.direction {
                    Some(direction) => Ok(PluginCommand::MoveFocus(direction.try_into()?)),
                    None => Err("Malformed move focus payload"),
                },
                _ => Err("Mismatched payload for MoveFocus"),
            },
            Some(CommandName::MoveFocusOrTab) => match protobuf_plugin_command.payload {
                Some(Payload::MoveFocusOrTabPayload(move_payload)) => {
                    match move_payload.direction {
                        Some(direction) => Ok(PluginCommand::MoveFocusOrTab(direction.try_into()?)),
                        None => Err("Malformed move focus or tab payload"),
                    }
                },
                _ => Err("Mismatched payload for MoveFocusOrTab"),
            },
            Some(CommandName::Detach) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("Detach should not have a payload");
                }
                Ok(PluginCommand::Detach)
            },
            Some(CommandName::EditScrollback) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("EditScrollback should not have a payload");
                }
                Ok(PluginCommand::EditScrollback)
            },
            Some(CommandName::Write) => match protobuf_plugin_command.payload {
                Some(Payload::WritePayload(bytes)) => Ok(PluginCommand::Write(bytes)),
                _ => Err("Mismatched payload for Write"),
            },
            Some(CommandName::WriteChars) => match protobuf_plugin_command.payload {
                Some(Payload::WriteCharsPayload(chars)) => Ok(PluginCommand::WriteChars(chars)),
                _ => Err("Mismatched payload for WriteChars"),
            },
            Some(CommandName::ToggleTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ToggleTab should not have a payload");
                }
                Ok(PluginCommand::ToggleTab)
            },
            Some(CommandName::MovePane) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("MovePane should not have a payload");
                }
                Ok(PluginCommand::MovePane)
            },
            Some(CommandName::MovePaneWithDirection) => match protobuf_plugin_command.payload {
                Some(Payload::MovePaneWithDirectionPayload(move_payload)) => {
                    match move_payload.direction {
                        Some(direction) => {
                            Ok(PluginCommand::MovePaneWithDirection(direction.try_into()?))
                        },
                        None => Err("Malformed MovePaneWithDirection payload"),
                    }
                },
                _ => Err("Mismatched payload for MovePaneWithDirection"),
            },
            Some(CommandName::ClearScreen) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ClearScreen should not have a payload");
                }
                Ok(PluginCommand::ClearScreen)
            },
            Some(CommandName::ScrollUp) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ScrollUp should not have a payload");
                }
                Ok(PluginCommand::ScrollUp)
            },
            Some(CommandName::ScrollDown) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ScrollDown should not have a payload");
                }
                Ok(PluginCommand::ScrollDown)
            },
            Some(CommandName::ScrollToTop) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ScrollToTop should not have a payload");
                }
                Ok(PluginCommand::ScrollToTop)
            },
            Some(CommandName::ScrollToBottom) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ScrollToBottom should not have a payload");
                }
                Ok(PluginCommand::ScrollToBottom)
            },
            Some(CommandName::PageScrollUp) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("PageScrollUp should not have a payload");
                }
                Ok(PluginCommand::PageScrollUp)
            },
            Some(CommandName::PageScrollDown) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("PageScrollDown should not have a payload");
                }
                Ok(PluginCommand::PageScrollDown)
            },
            Some(CommandName::ToggleFocusFullscreen) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ToggleFocusFullscreen should not have a payload");
                }
                Ok(PluginCommand::ToggleFocusFullscreen)
            },
            Some(CommandName::TogglePaneFrames) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("TogglePaneFrames should not have a payload");
                }
                Ok(PluginCommand::TogglePaneFrames)
            },
            Some(CommandName::TogglePaneEmbedOrEject) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("TogglePaneEmbedOrEject should not have a payload");
                }
                Ok(PluginCommand::TogglePaneEmbedOrEject)
            },
            Some(CommandName::UndoRenamePane) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("UndoRenamePane should not have a payload");
                }
                Ok(PluginCommand::UndoRenamePane)
            },
            Some(CommandName::CloseFocus) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("CloseFocus should not have a payload");
                }
                Ok(PluginCommand::CloseFocus)
            },
            Some(CommandName::ToggleActiveTabSync) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("ToggleActiveTabSync should not have a payload");
                }
                Ok(PluginCommand::ToggleActiveTabSync)
            },
            Some(CommandName::CloseFocusedTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("CloseFocusedTab should not have a payload");
                }
                Ok(PluginCommand::CloseFocusedTab)
            },
            Some(CommandName::UndoRenameTab) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("UndoRenameTab should not have a payload");
                }
                Ok(PluginCommand::UndoRenameTab)
            },
            Some(CommandName::QuitZellij) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("QuitZellij should not have a payload");
                }
                Ok(PluginCommand::QuitZellij)
            },
            Some(CommandName::PreviousSwapLayout) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("PreviousSwapLayout should not have a payload");
                }
                Ok(PluginCommand::PreviousSwapLayout)
            },
            Some(CommandName::NextSwapLayout) => {
                if protobuf_plugin_command.payload.is_some() {
                    return Err("NextSwapLayout should not have a payload");
                }
                Ok(PluginCommand::NextSwapLayout)
            },
            Some(CommandName::GoToTabName) => match protobuf_plugin_command.payload {
                Some(Payload::GoToTabNamePayload(tab_name)) => {
                    Ok(PluginCommand::GoToTabName(tab_name))
                },
                _ => Err("Mismatched payload for GoToTabName"),
            },
            Some(CommandName::FocusOrCreateTab) => match protobuf_plugin_command.payload {
                Some(Payload::FocusOrCreateTabPayload(tab_name)) => {
                    Ok(PluginCommand::FocusOrCreateTab(tab_name))
                },
                _ => Err("Mismatched payload for FocusOrCreateTab"),
            },
            Some(CommandName::GoToTab) => match protobuf_plugin_command.payload {
                Some(Payload::GoToTabPayload(tab_index)) => {
                    Ok(PluginCommand::GoToTab(tab_index as u32))
                },
                _ => Err("Mismatched payload for GoToTab"),
            },
            Some(CommandName::StartOrReloadPlugin) => match protobuf_plugin_command.payload {
                Some(Payload::StartOrReloadPluginPayload(url)) => {
                    Ok(PluginCommand::StartOrReloadPlugin(url))
                },
                _ => Err("Mismatched payload for StartOrReloadPlugin"),
            },
            Some(CommandName::CloseTerminalPane) => match protobuf_plugin_command.payload {
                Some(Payload::CloseTerminalPanePayload(pane_id)) => {
                    Ok(PluginCommand::CloseTerminalPane(pane_id as u32))
                },
                _ => Err("Mismatched payload for CloseTerminalPane"),
            },
            Some(CommandName::ClosePluginPane) => match protobuf_plugin_command.payload {
                Some(Payload::ClosePluginPanePayload(pane_id)) => {
                    Ok(PluginCommand::ClosePluginPane(pane_id as u32))
                },
                _ => Err("Mismatched payload for ClosePluginPane"),
            },
            Some(CommandName::FocusTerminalPane) => match protobuf_plugin_command.payload {
                Some(Payload::FocusTerminalPanePayload(payload)) => {
                    let pane_id = payload.pane_id as u32;
                    let should_float = payload.should_float;
                    Ok(PluginCommand::FocusTerminalPane(pane_id, should_float))
                },
                _ => Err("Mismatched payload for ClosePluginPane"),
            },
            Some(CommandName::FocusPluginPane) => match protobuf_plugin_command.payload {
                Some(Payload::FocusPluginPanePayload(payload)) => {
                    let pane_id = payload.pane_id as u32;
                    let should_float = payload.should_float;
                    Ok(PluginCommand::FocusPluginPane(pane_id, should_float))
                },
                _ => Err("Mismatched payload for ClosePluginPane"),
            },
            Some(CommandName::RenameTerminalPane) => match protobuf_plugin_command.payload {
                Some(Payload::RenameTerminalPanePayload(payload)) => {
                    let pane_id = payload.id as u32;
                    let new_name = payload.new_name;
                    Ok(PluginCommand::RenameTerminalPane(pane_id, new_name))
                },
                _ => Err("Mismatched payload for RenameTerminalPane"),
            },
            Some(CommandName::RenamePluginPane) => match protobuf_plugin_command.payload {
                Some(Payload::RenamePluginPanePayload(payload)) => {
                    let pane_id = payload.id as u32;
                    let new_name = payload.new_name;
                    Ok(PluginCommand::RenamePluginPane(pane_id, new_name))
                },
                _ => Err("Mismatched payload for RenamePluginPane"),
            },
            Some(CommandName::RenameTab) => match protobuf_plugin_command.payload {
                Some(Payload::RenameTabPayload(payload)) => {
                    let tab_index = payload.id as u32;
                    let name = payload.new_name;
                    Ok(PluginCommand::RenameTab(tab_index, name))
                },
                _ => Err("Mismatched payload for RenameTab"),
            },
            Some(CommandName::ReportCrash) => match protobuf_plugin_command.payload {
                Some(Payload::ReportCrashPayload(payload)) => {
                    Ok(PluginCommand::ReportPanic(payload))
                },
                _ => Err("Mismatched payload for ReportCrash"),
            },
            Some(CommandName::RequestPluginPermissions) => match protobuf_plugin_command.payload {
                Some(Payload::RequestPluginPermissionPayload(payload)) => {
                    Ok(PluginCommand::RequestPluginPermissions(
                        payload
                            .permissions
                            .iter()
                            .filter_map(|p| ProtobufPermissionType::from_i32(*p))
                            .filter_map(|p| PermissionType::try_from(p).ok())
                            .collect(),
                    ))
                },
                _ => Err("Mismatched payload for RequestPluginPermission"),
            },
            Some(CommandName::SwitchSession) => match protobuf_plugin_command.payload {
                Some(Payload::SwitchSessionPayload(payload)) => {
                    let pane_id = match (payload.pane_id, payload.pane_id_is_plugin) {
                        (Some(pane_id), Some(is_plugin)) => Some((pane_id, is_plugin)),
                        (None, None) => None,
                        _ => {
                            return Err("Malformed payload for SwitchSession, 'pane_id' and 'is_plugin' must be included together or not at all")
                        }
                    };
                    Ok(PluginCommand::SwitchSession(ConnectToSession {
                        name: payload.name,
                        tab_position: payload.tab_position.map(|p| p as usize),
                        pane_id,
                    }))
                },
                _ => Err("Mismatched payload for SwitchSession"),
            },
            None => Err("Unrecognized plugin command"),
        }
    }
}

impl TryFrom<PluginCommand> for ProtobufPluginCommand {
    type Error = &'static str;
    fn try_from(plugin_command: PluginCommand) -> Result<Self, &'static str> {
        match plugin_command {
            PluginCommand::Subscribe(subscriptions) => {
                let subscriptions: ProtobufEventNameList = subscriptions.try_into()?;
                Ok(ProtobufPluginCommand {
                    name: CommandName::Subscribe as i32,
                    payload: Some(Payload::SubscribePayload(SubscribePayload {
                        subscriptions: Some(subscriptions),
                    })),
                })
            },
            PluginCommand::Unsubscribe(subscriptions) => {
                let subscriptions: ProtobufEventNameList = subscriptions.try_into()?;
                Ok(ProtobufPluginCommand {
                    name: CommandName::Unsubscribe as i32,
                    payload: Some(Payload::UnsubscribePayload(UnsubscribePayload {
                        subscriptions: Some(subscriptions),
                    })),
                })
            },
            PluginCommand::SetSelectable(should_be_selectable) => Ok(ProtobufPluginCommand {
                name: CommandName::SetSelectable as i32,
                payload: Some(Payload::SetSelectablePayload(should_be_selectable)),
            }),
            PluginCommand::GetPluginIds => Ok(ProtobufPluginCommand {
                name: CommandName::GetPluginIds as i32,
                payload: None,
            }),
            PluginCommand::GetZellijVersion => Ok(ProtobufPluginCommand {
                name: CommandName::GetZellijVersion as i32,
                payload: None,
            }),
            PluginCommand::OpenFile(file_to_open) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenFile as i32,
                payload: Some(Payload::OpenFilePayload(OpenFilePayload {
                    file_to_open: Some(file_to_open.try_into()?),
                })),
            }),
            PluginCommand::OpenFileFloating(file_to_open) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenFileFloating as i32,
                payload: Some(Payload::OpenFileFloatingPayload(OpenFilePayload {
                    file_to_open: Some(file_to_open.try_into()?),
                })),
            }),
            PluginCommand::OpenTerminal(cwd) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenTerminal as i32,
                payload: Some(Payload::OpenTerminalPayload(OpenFilePayload {
                    file_to_open: Some(cwd.try_into()?),
                })),
            }),
            PluginCommand::OpenTerminalFloating(cwd) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenTerminalFloating as i32,
                payload: Some(Payload::OpenTerminalFloatingPayload(OpenFilePayload {
                    file_to_open: Some(cwd.try_into()?),
                })),
            }),
            PluginCommand::OpenCommandPane(command_to_run) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenCommandPane as i32,
                payload: Some(Payload::OpenCommandPanePayload(OpenCommandPanePayload {
                    command_to_run: Some(command_to_run.try_into()?),
                })),
            }),
            PluginCommand::OpenCommandPaneFloating(command_to_run) => Ok(ProtobufPluginCommand {
                name: CommandName::OpenCommandPaneFloating as i32,
                payload: Some(Payload::OpenCommandPaneFloatingPayload(
                    OpenCommandPanePayload {
                        command_to_run: Some(command_to_run.try_into()?),
                    },
                )),
            }),
            PluginCommand::SwitchTabTo(tab_index) => Ok(ProtobufPluginCommand {
                name: CommandName::SwitchTabTo as i32,
                payload: Some(Payload::SwitchTabToPayload(SwitchTabToPayload {
                    tab_index: tab_index as i32,
                })),
            }),
            PluginCommand::SetTimeout(seconds) => Ok(ProtobufPluginCommand {
                name: CommandName::SetTimeout as i32,
                payload: Some(Payload::SetTimeoutPayload(SetTimeoutPayload { seconds })),
            }),
            PluginCommand::ExecCmd(command_line) => Ok(ProtobufPluginCommand {
                name: CommandName::ExecCmd as i32,
                payload: Some(Payload::ExecCmdPayload(ExecCmdPayload { command_line })),
            }),
            PluginCommand::PostMessageTo(plugin_message) => Ok(ProtobufPluginCommand {
                name: CommandName::PostMessageTo as i32,
                payload: Some(Payload::PostMessageToPayload(PluginMessagePayload {
                    message: Some(plugin_message.try_into()?),
                })),
            }),
            PluginCommand::PostMessageToPlugin(plugin_message) => Ok(ProtobufPluginCommand {
                name: CommandName::PostMessageToPlugin as i32,
                payload: Some(Payload::PostMessageToPluginPayload(PluginMessagePayload {
                    message: Some(plugin_message.try_into()?),
                })),
            }),
            PluginCommand::HideSelf => Ok(ProtobufPluginCommand {
                name: CommandName::HideSelf as i32,
                payload: None,
            }),
            PluginCommand::ShowSelf(should_float_if_hidden) => Ok(ProtobufPluginCommand {
                name: CommandName::ShowSelf as i32,
                payload: Some(Payload::ShowSelfPayload(should_float_if_hidden)),
            }),
            PluginCommand::SwitchToMode(input_mode) => Ok(ProtobufPluginCommand {
                name: CommandName::SwitchToMode as i32,
                payload: Some(Payload::SwitchToModePayload(SwitchToModePayload {
                    input_mode: Some(input_mode.try_into()?),
                })),
            }),
            PluginCommand::NewTabsWithLayout(raw_layout) => Ok(ProtobufPluginCommand {
                name: CommandName::NewTabsWithLayout as i32,
                payload: Some(Payload::NewTabsWithLayoutPayload(raw_layout)),
            }),
            PluginCommand::NewTab => Ok(ProtobufPluginCommand {
                name: CommandName::NewTab as i32,
                payload: None,
            }),
            PluginCommand::GoToNextTab => Ok(ProtobufPluginCommand {
                name: CommandName::GoToNextTab as i32,
                payload: None,
            }),
            PluginCommand::GoToPreviousTab => Ok(ProtobufPluginCommand {
                name: CommandName::GoToPreviousTab as i32,
                payload: None,
            }),
            PluginCommand::Resize(resize) => Ok(ProtobufPluginCommand {
                name: CommandName::Resize as i32,
                payload: Some(Payload::ResizePayload(ResizePayload {
                    resize: Some(resize.try_into()?),
                })),
            }),
            PluginCommand::ResizeWithDirection(resize) => Ok(ProtobufPluginCommand {
                name: CommandName::ResizeWithDirection as i32,
                payload: Some(Payload::ResizeWithDirectionPayload(ResizePayload {
                    resize: Some(resize.try_into()?),
                })),
            }),
            PluginCommand::FocusNextPane => Ok(ProtobufPluginCommand {
                name: CommandName::FocusNextPane as i32,
                payload: None,
            }),
            PluginCommand::FocusPreviousPane => Ok(ProtobufPluginCommand {
                name: CommandName::FocusPreviousPane as i32,
                payload: None,
            }),
            PluginCommand::MoveFocus(direction) => Ok(ProtobufPluginCommand {
                name: CommandName::MoveFocus as i32,
                payload: Some(Payload::MoveFocusPayload(MovePayload {
                    direction: Some(direction.try_into()?),
                })),
            }),
            PluginCommand::MoveFocusOrTab(direction) => Ok(ProtobufPluginCommand {
                name: CommandName::MoveFocusOrTab as i32,
                payload: Some(Payload::MoveFocusOrTabPayload(MovePayload {
                    direction: Some(direction.try_into()?),
                })),
            }),
            PluginCommand::Detach => Ok(ProtobufPluginCommand {
                name: CommandName::Detach as i32,
                payload: None,
            }),
            PluginCommand::EditScrollback => Ok(ProtobufPluginCommand {
                name: CommandName::EditScrollback as i32,
                payload: None,
            }),
            PluginCommand::Write(bytes) => Ok(ProtobufPluginCommand {
                name: CommandName::Write as i32,
                payload: Some(Payload::WritePayload(bytes)),
            }),
            PluginCommand::WriteChars(chars) => Ok(ProtobufPluginCommand {
                name: CommandName::WriteChars as i32,
                payload: Some(Payload::WriteCharsPayload(chars)),
            }),
            PluginCommand::ToggleTab => Ok(ProtobufPluginCommand {
                name: CommandName::ToggleTab as i32,
                payload: None,
            }),
            PluginCommand::MovePane => Ok(ProtobufPluginCommand {
                name: CommandName::MovePane as i32,
                payload: None,
            }),
            PluginCommand::MovePaneWithDirection(direction) => Ok(ProtobufPluginCommand {
                name: CommandName::MovePaneWithDirection as i32,
                payload: Some(Payload::MovePaneWithDirectionPayload(MovePayload {
                    direction: Some(direction.try_into()?),
                })),
            }),
            PluginCommand::ClearScreen => Ok(ProtobufPluginCommand {
                name: CommandName::ClearScreen as i32,
                payload: None,
            }),
            PluginCommand::ScrollUp => Ok(ProtobufPluginCommand {
                name: CommandName::ScrollUp as i32,
                payload: None,
            }),
            PluginCommand::ScrollDown => Ok(ProtobufPluginCommand {
                name: CommandName::ScrollDown as i32,
                payload: None,
            }),
            PluginCommand::ScrollToTop => Ok(ProtobufPluginCommand {
                name: CommandName::ScrollToTop as i32,
                payload: None,
            }),
            PluginCommand::ScrollToBottom => Ok(ProtobufPluginCommand {
                name: CommandName::ScrollToBottom as i32,
                payload: None,
            }),
            PluginCommand::PageScrollUp => Ok(ProtobufPluginCommand {
                name: CommandName::PageScrollUp as i32,
                payload: None,
            }),
            PluginCommand::PageScrollDown => Ok(ProtobufPluginCommand {
                name: CommandName::PageScrollDown as i32,
                payload: None,
            }),
            PluginCommand::ToggleFocusFullscreen => Ok(ProtobufPluginCommand {
                name: CommandName::ToggleFocusFullscreen as i32,
                payload: None,
            }),
            PluginCommand::TogglePaneFrames => Ok(ProtobufPluginCommand {
                name: CommandName::TogglePaneFrames as i32,
                payload: None,
            }),
            PluginCommand::TogglePaneEmbedOrEject => Ok(ProtobufPluginCommand {
                name: CommandName::TogglePaneEmbedOrEject as i32,
                payload: None,
            }),
            PluginCommand::UndoRenamePane => Ok(ProtobufPluginCommand {
                name: CommandName::UndoRenamePane as i32,
                payload: None,
            }),
            PluginCommand::CloseFocus => Ok(ProtobufPluginCommand {
                name: CommandName::CloseFocus as i32,
                payload: None,
            }),
            PluginCommand::ToggleActiveTabSync => Ok(ProtobufPluginCommand {
                name: CommandName::ToggleActiveTabSync as i32,
                payload: None,
            }),
            PluginCommand::CloseFocusedTab => Ok(ProtobufPluginCommand {
                name: CommandName::CloseFocusedTab as i32,
                payload: None,
            }),
            PluginCommand::UndoRenameTab => Ok(ProtobufPluginCommand {
                name: CommandName::UndoRenameTab as i32,
                payload: None,
            }),
            PluginCommand::QuitZellij => Ok(ProtobufPluginCommand {
                name: CommandName::QuitZellij as i32,
                payload: None,
            }),
            PluginCommand::PreviousSwapLayout => Ok(ProtobufPluginCommand {
                name: CommandName::PreviousSwapLayout as i32,
                payload: None,
            }),
            PluginCommand::NextSwapLayout => Ok(ProtobufPluginCommand {
                name: CommandName::NextSwapLayout as i32,
                payload: None,
            }),
            PluginCommand::GoToTabName(tab_name) => Ok(ProtobufPluginCommand {
                name: CommandName::GoToTabName as i32,
                payload: Some(Payload::GoToTabNamePayload(tab_name)),
            }),
            PluginCommand::FocusOrCreateTab(tab_name) => Ok(ProtobufPluginCommand {
                name: CommandName::FocusOrCreateTab as i32,
                payload: Some(Payload::FocusOrCreateTabPayload(tab_name)),
            }),
            PluginCommand::GoToTab(tab_index) => Ok(ProtobufPluginCommand {
                name: CommandName::GoToTab as i32,
                payload: Some(Payload::GoToTabPayload(tab_index as i32)),
            }),
            PluginCommand::StartOrReloadPlugin(url) => Ok(ProtobufPluginCommand {
                name: CommandName::StartOrReloadPlugin as i32,
                payload: Some(Payload::StartOrReloadPluginPayload(url)),
            }),
            PluginCommand::CloseTerminalPane(pane_id) => Ok(ProtobufPluginCommand {
                name: CommandName::CloseTerminalPane as i32,
                payload: Some(Payload::CloseTerminalPanePayload(pane_id as i32)),
            }),
            PluginCommand::ClosePluginPane(pane_id) => Ok(ProtobufPluginCommand {
                name: CommandName::ClosePluginPane as i32,
                payload: Some(Payload::ClosePluginPanePayload(pane_id as i32)),
            }),
            PluginCommand::FocusTerminalPane(pane_id, should_float_if_hidden) => {
                Ok(ProtobufPluginCommand {
                    name: CommandName::FocusTerminalPane as i32,
                    payload: Some(Payload::FocusTerminalPanePayload(PaneIdAndShouldFloat {
                        pane_id: pane_id as i32,
                        should_float: should_float_if_hidden,
                    })),
                })
            },
            PluginCommand::FocusPluginPane(pane_id, should_float_if_hidden) => {
                Ok(ProtobufPluginCommand {
                    name: CommandName::FocusPluginPane as i32,
                    payload: Some(Payload::FocusPluginPanePayload(PaneIdAndShouldFloat {
                        pane_id: pane_id as i32,
                        should_float: should_float_if_hidden,
                    })),
                })
            },
            PluginCommand::RenameTerminalPane(pane_id, new_name) => Ok(ProtobufPluginCommand {
                name: CommandName::RenameTerminalPane as i32,
                payload: Some(Payload::RenameTerminalPanePayload(IdAndNewName {
                    id: pane_id as i32,
                    new_name,
                })),
            }),
            PluginCommand::RenamePluginPane(pane_id, new_name) => Ok(ProtobufPluginCommand {
                name: CommandName::RenamePluginPane as i32,
                payload: Some(Payload::RenamePluginPanePayload(IdAndNewName {
                    id: pane_id as i32,
                    new_name,
                })),
            }),
            PluginCommand::RenameTab(tab_index, new_name) => Ok(ProtobufPluginCommand {
                name: CommandName::RenameTab as i32,
                payload: Some(Payload::RenameTabPayload(IdAndNewName {
                    id: tab_index as i32,
                    new_name,
                })),
            }),
            PluginCommand::ReportPanic(payload) => Ok(ProtobufPluginCommand {
                name: CommandName::ReportCrash as i32,
                payload: Some(Payload::ReportCrashPayload(payload)),
            }),
            PluginCommand::RequestPluginPermissions(permissions) => Ok(ProtobufPluginCommand {
                name: CommandName::RequestPluginPermissions as i32,
                payload: Some(Payload::RequestPluginPermissionPayload(
                    RequestPluginPermissionPayload {
                        permissions: permissions
                            .iter()
                            .filter_map(|p| ProtobufPermissionType::try_from(*p).ok())
                            .map(|p| p as i32)
                            .collect(),
                    },
                )),
            }),
            PluginCommand::SwitchSession(switch_to_session) => Ok(ProtobufPluginCommand {
                name: CommandName::SwitchSession as i32,
                payload: Some(Payload::SwitchSessionPayload(
                    SwitchSessionPayload {
                        name: switch_to_session.name,
                        tab_position: switch_to_session.tab_position.map(|t| t as u32),
                        pane_id: switch_to_session.pane_id.map(|p| p.0),
                        pane_id_is_plugin: switch_to_session.pane_id.map(|p| p.1),
                    },
                )),
            }),
        }
    }
}

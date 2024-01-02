pub mod subscription;
pub mod socket_communication;
pub mod bspc;

#[cfg(test)]
mod tests {
    use crate::{bspc::{node::command::NodeCommand, node::{command::assemble_node_command as assemble_node_command, states::NodeState, selection::{NodeModifiers, NodeDescriptor, NodeSelector, assemble_selector}}, query::{QueryCommand, assemble_query_command as assemble_query}, desktop::{command::{DesktopCommand, self, assemble_desktop_command as assemble_desktop_command}, selection::{DesktopModifier, DesktopSelector, DesktopDescriptor}}}, socket_communication::{send_message, get_bspc_socket_path}, subscription::subscribe};

    #[test]
    fn test() {
        let reference: NodeSelector = NodeSelector {
            reference_selector: None,
            descriptor: Some(NodeDescriptor::Biggest),
            modifiers: vec![]
        };
        let command: NodeCommand = NodeCommand::Focus(NodeSelector {
            reference_selector: Some(assemble_selector(reference)),
            descriptor: Some(NodeDescriptor::Older),
            modifiers: vec![NodeModifiers::NotFocused]
        });
        let command: Vec<String> = assemble_node_command(command);

        let result = send_message(get_bspc_socket_path(), command);
        match result {
            Some(lines) => {
                for line in lines {
                    println!("response: {:?}", line);
                }
            }
            None => {
                println!("No response");
            }
        }
    }

    #[test]
    fn test2() {
        let command: DesktopCommand = DesktopCommand::Focus(DesktopSelector {
            reference_selector: None,
            descriptor: Some(DesktopDescriptor::Last),
            modifiers: vec![]
        });
        let command: Vec<String> = assemble_desktop_command(command);

        let result = send_message(get_bspc_socket_path(), command);
        match result {
            Some(lines) => {
                for line in lines {
                    println!("response: {:?}", line);
                }
            }
            None => {
                println!("No response");
            }
        }
    }

    #[test]
    fn test3() {
        subscribe("node_add".to_string(), callback, Vec::new())
    }

    fn callback(_args: Vec<&str>, _callback_args: Vec<&str>) {
        let command: NodeCommand = NodeCommand::State(NodeState::Fullscreen);
        let command = assemble_node_command(command);
        send_message(get_bspc_socket_path(), command);
    }
}

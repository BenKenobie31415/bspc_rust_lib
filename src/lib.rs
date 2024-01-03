pub mod bspc;
pub mod socket_communication;
pub mod subscription;
pub mod util;

// TODO make commands structs (maybe not sure just an idea)

#[cfg(test)]
mod tests {
    use crate::{
        bspc::{
            node::{selection::{NodeModifier, NodeSelector}, command::NodeCommand, states::NodeState},
            query::QueryCommand,
        },
        socket_communication::{get_bspc_socket_path, send_message}, util::get_class_name_from_id,
    };

    #[test]
    fn test() {
        let command: QueryCommand = QueryCommand::Nodes(
            Some(NodeSelector {
                reference_selector: None,
                descriptor: None,
                modifiers: vec![NodeModifier::Focused]}), None, None);
        let command = command.assemble();
        let result = send_message(get_bspc_socket_path(), command);
        let node: String;
        match result {
            Some(output) => {
                node = output[0].clone();
            },
            None => {
                node = "".to_string();
            }
        }
        let node_name = get_class_name_from_id(&node);
        println!("node: {:?}", node_name);
    }

    fn test2() {
        let command: NodeCommand = NodeCommand::State(NodeState::Fullscreen);
        let result = command.get_response();
    }
}

use brack_plugin::types::Type;
pub enum OpCode {
    ToArray(usize),
    ToOption(Option<()>),
    Join(usize),
    Call {
        plugin_name: String,
        function_name: String,
        return_type: Type,
    },
    Push(String),
}

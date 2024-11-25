use brack_plugin::types::Type;
pub enum OpCode {
    ToArray(usize),
    ToOption,
    Call {
        plugin_name: String,
        function_name: String,
        return_type: Type,
    },
    Push(String),
}

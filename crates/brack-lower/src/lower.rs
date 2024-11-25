use anyhow::Result;
use brack_plugin::plugins::Plugins;
use brack_transformer::ast::AST;

use crate::op_code::OpCode;
use crate::{curly, expr, square, stmt};

pub fn lowering(ast: &AST, plugins: Plugins) -> Result<Vec<OpCode>> {
    match ast {
        AST::Document(_) => (),
        _ => anyhow::bail!("Document must be a document"),
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::lowering(child, &plugins)?,
            AST::Expr(_) => expr::lowering(child, &plugins)?,
            AST::Curly(_) => curly::lowering(child, &plugins)?,
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Document cannot contain the following node\n{}", ast),
        };
        result.extend(res);
    }
    Ok(result)
}

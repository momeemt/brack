use anyhow::Result;
use brack_plugin::plugins::Plugins;
use brack_transformer::ast::AST;

use crate::op_code::OpCode;
use crate::{expr, square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>> {
    match ast {
        AST::Curly(_) => (),
        _ => anyhow::bail!("Curly must be a curly"),
    }

    let mut op_codes = vec![];
    let module = ast
        .children()
        .first()
        .ok_or_else(|| anyhow::anyhow!("Curly must contain module"))?;

    let ident = ast
        .children()
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Curly must contain identifier"))?;

    for child in ast.children().iter().skip(2) {
        let res = match child {
            AST::Expr(_) => expr::lowering(child, &plugins)?,
            AST::Curly(_) => lowering(child, &plugins)?,
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Text(_) => text::lowering(child, &plugins)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Curly cannot contain the following node\n{}", ast),
        };
        op_codes.extend(res);
    }

    op_codes.push(OpCode::Join(ast.children().len() - 2));

    Ok(op_codes)
}

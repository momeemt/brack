use brack_parser::cst::{InnerNode, CST};
use brack_tokenizer::tokens::merge_location;

use crate::{bracket::{check_if_dot, check_if_ident_or_angle_bracket, check_if_module_or_angle_bracket, check_unexpected_dot, remove_elements_not_included_ast}, error::TransformError, transform};

fn check_if_the_first_and_last_node_are_brackets(csts: &Vec<CST>) -> Vec<TransformError> {
    let mut errors = vec![];
    match (csts[0].clone(), csts[csts.len() - 1].clone()) {
        (CST::AngleBracketOpen(_), CST::AngleBracketClose(_)) => (),
        (CST::AngleBracketOpen(left), CST::CurlyBracketClose(right))
        | (CST::AngleBracketOpen(left), CST::SquareBracketClose(right)) => errors.push(
            TransformError::MismatchedBracket(merge_location(&left.location, &right.location)),
        ),
        (CST::AngleBracketOpen(left), right) => errors.push(TransformError::AngleNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        _ => panic!(
            "Maybe cst parser is broken because CST::Angle mush have bracket-open node first."
        ),
    }
    errors
}

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::Angle(node) => node,
        _ => panic!("Cannot pass non-angle-bracket node to angle::simplify"),
    };
    let mut errors = vec![];
    let mut csts = vec![];
    for child in node.children.clone() {
        let (cst, mut node_errors) = transform::transform(&child);
        csts.push(cst);
        errors.append(&mut node_errors);
    }

    errors.append(&mut check_if_the_first_and_last_node_are_brackets(&csts));
    errors.append(&mut check_if_module_or_angle_bracket(&csts));
    errors.append(&mut check_if_dot(&csts));
    errors.append(&mut check_if_ident_or_angle_bracket(&csts));
    errors.append(&mut check_unexpected_dot(&csts));

    // commaで区切ってexprにまとめる

    let csts = remove_elements_not_included_ast(&csts);
    (
        CST::Angle(InnerNode {
            id: node.id.clone(),
            children: csts,
            location: node.location.clone(),
        }),
        errors,
    )
}

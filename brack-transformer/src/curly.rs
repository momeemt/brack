use brack_parser::cst::CST;
use brack_tokenizer::tokens::merge_location;

use crate::error::TransformError;

fn check_if_the_first_and_last_node_are_brackets(csts: &Vec<CST>) -> Vec<TransformError> {
    let mut errors = vec![];
    match (csts[0].clone(), csts[csts.len() - 1].clone()) {
        (CST::CurlyBracketOpen(_), CST::CurlyBracketClose(_)) => (),
        (CST::CurlyBracketOpen(left), CST::AngleBracketClose(right))
        | (CST::CurlyBracketOpen(left), CST::SquareBracketClose(right)) => errors.push(
            TransformError::MismatchedBracket(merge_location(&left.location, &right.location)),
        ),
        (CST::CurlyBracketOpen(left), right) => errors.push(TransformError::CurlyNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        _ => panic!(
            "Maybe cst parser is broken because CST::Angle mush have bracket-open node first."
        ),
    }
    errors
}

pub fn simplify(_cst: &CST) -> (CST, Vec<TransformError>) {
    todo!()
}

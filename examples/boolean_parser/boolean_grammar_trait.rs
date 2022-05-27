// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;

use miette::{miette, Result};
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

use crate::boolean_grammar::BooleanGrammar;
use std::path::Path;

///
/// The `BooleanGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait BooleanGrammarTrait {
    ///
    /// Implement this method if you need the provided information
    ///
    fn init(&mut self, _file_name: &Path) {}

    /// Semantic action for production 0:
    ///
    /// Expressions: Expression ExpressionsList /* Vec */ ExpressionsSuffix;
    ///
    fn expressions(
        &mut self,
        _expression: &ParseTreeStackEntry,
        _expressions_list: &ParseTreeStackEntry,
        _expressions_suffix: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// ExpressionsSuffix: Semicolon;
    ///
    fn expressions_suffix_0(
        &mut self,
        _semicolon: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ExpressionsSuffix: ;
    ///
    fn expressions_suffix_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ExpressionsList: Semicolon Expression ExpressionsList; // Vec<T>::Push
    ///
    fn expressions_list_0(
        &mut self,
        _semicolon: &ParseTreeStackEntry,
        _expression: &ParseTreeStackEntry,
        _expressions_list: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ExpressionsList: ; // Vec<T>::New
    ///
    fn expressions_list_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// Expression: Term TailExpression;
    ///
    fn expression(
        &mut self,
        _term: &ParseTreeStackEntry,
        _tail_expression: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// TailExpression: TailExpressionList /* Vec */;
    ///
    fn tail_expression(
        &mut self,
        _tail_expression_list: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// TailExpressionList: BinaryOperator Term TailExpressionList; // Vec<T>::Push
    ///
    fn tail_expression_list_0(
        &mut self,
        _binary_operator: &ParseTreeStackEntry,
        _term: &ParseTreeStackEntry,
        _tail_expression_list: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// TailExpressionList: ; // Vec<T>::New
    ///
    fn tail_expression_list_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// Term: TermOpt Factor;
    ///
    fn term(
        &mut self,
        _term_opt: &ParseTreeStackEntry,
        _factor: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// TermOpt: UnaryOperator;
    ///
    fn term_opt_0(
        &mut self,
        _unary_operator: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// TermOpt: ;
    ///
    fn term_opt_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// Boolean: True;
    ///
    fn boolean_0(
        &mut self,
        _true: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// Boolean: False;
    ///
    fn boolean_1(
        &mut self,
        _false: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// UnaryOperator: Not;
    ///
    fn unary_operator(
        &mut self,
        _not: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// BinaryOperator: AndOp;
    ///
    fn binary_operator_0(
        &mut self,
        _and_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// BinaryOperator: OrOp;
    ///
    fn binary_operator_1(
        &mut self,
        _or_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// BinaryOperator: XorOp;
    ///
    fn binary_operator_2(
        &mut self,
        _xor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// BinaryOperator: NorOp;
    ///
    fn binary_operator_3(
        &mut self,
        _nor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// BinaryOperator: NandOp;
    ///
    fn binary_operator_4(
        &mut self,
        _nand_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// BinaryOperator: XnorOp;
    ///
    fn binary_operator_5(
        &mut self,
        _xnor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// AndOp: "(?i)AND";
    ///
    fn and_op(
        &mut self,
        _and_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// OrOp: "(?i)OR";
    ///
    fn or_op(
        &mut self,
        _or_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// XorOp: "(?i)XOR";
    ///
    fn xor_op(
        &mut self,
        _xor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// NorOp: "(?i)NOR";
    ///
    fn nor_op(
        &mut self,
        _nor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// NandOp: "(?i)NAND";
    ///
    fn nand_op(
        &mut self,
        _nand_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// XnorOp: "(?i)XNOR";
    ///
    fn xnor_op(
        &mut self,
        _xnor_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 27:
    ///
    /// True: "(?i)TRUE";
    ///
    fn r#true(
        &mut self,
        _true: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 28:
    ///
    /// False: "(?i)FALSE";
    ///
    fn r#false(
        &mut self,
        _false: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 29:
    ///
    /// Not: "(?i)NOT";
    ///
    fn not(&mut self, _not: &ParseTreeStackEntry, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 30:
    ///
    /// Parenthesized: LeftParenthesis Expression RightParenthesis;
    ///
    fn parenthesized(
        &mut self,
        _left_parenthesis: &ParseTreeStackEntry,
        _expression: &ParseTreeStackEntry,
        _right_parenthesis: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 31:
    ///
    /// Semicolon: ";";
    ///
    fn semicolon(
        &mut self,
        _semicolon: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 32:
    ///
    /// LeftParenthesis: "\(";
    ///
    fn left_parenthesis(
        &mut self,
        _left_parenthesis: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// RightParenthesis: "\)";
    ///
    fn right_parenthesis(
        &mut self,
        _right_parenthesis: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 34:
    ///
    /// Factor: Boolean;
    ///
    fn factor_0(
        &mut self,
        _boolean: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 35:
    ///
    /// Factor: Parenthesized;
    ///
    fn factor_1(
        &mut self,
        _parenthesized: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait<'_> for BooleanGrammar {
    ///
    /// Initialize the user with additional information.
    /// This function is called by the parser before parsing starts.
    /// It is used to transport necessary data from parser to user.
    ///
    fn init(&mut self, _file_name: &Path) {}

    ///
    /// This function is implemented automatically for the user's item BooleanGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.expressions(&children[0], &children[1], &children[2], parse_tree),
            1 => self.expressions_suffix_0(&children[0], parse_tree),
            2 => self.expressions_suffix_1(parse_tree),
            3 => self.expressions_list_0(&children[0], &children[1], &children[2], parse_tree),
            4 => self.expressions_list_1(parse_tree),
            5 => self.expression(&children[0], &children[1], parse_tree),
            6 => self.tail_expression(&children[0], parse_tree),
            7 => self.tail_expression_list_0(&children[0], &children[1], &children[2], parse_tree),
            8 => self.tail_expression_list_1(parse_tree),
            9 => self.term(&children[0], &children[1], parse_tree),
            10 => self.term_opt_0(&children[0], parse_tree),
            11 => self.term_opt_1(parse_tree),
            12 => self.boolean_0(&children[0], parse_tree),
            13 => self.boolean_1(&children[0], parse_tree),
            14 => self.unary_operator(&children[0], parse_tree),
            15 => self.binary_operator_0(&children[0], parse_tree),
            16 => self.binary_operator_1(&children[0], parse_tree),
            17 => self.binary_operator_2(&children[0], parse_tree),
            18 => self.binary_operator_3(&children[0], parse_tree),
            19 => self.binary_operator_4(&children[0], parse_tree),
            20 => self.binary_operator_5(&children[0], parse_tree),
            21 => self.and_op(&children[0], parse_tree),
            22 => self.or_op(&children[0], parse_tree),
            23 => self.xor_op(&children[0], parse_tree),
            24 => self.nor_op(&children[0], parse_tree),
            25 => self.nand_op(&children[0], parse_tree),
            26 => self.xnor_op(&children[0], parse_tree),
            27 => self.r#true(&children[0], parse_tree),
            28 => self.r#false(&children[0], parse_tree),
            29 => self.not(&children[0], parse_tree),
            30 => self.parenthesized(&children[0], &children[1], &children[2], parse_tree),
            31 => self.semicolon(&children[0], parse_tree),
            32 => self.left_parenthesis(&children[0], parse_tree),
            33 => self.right_parenthesis(&children[0], parse_tree),
            34 => self.factor_0(&children[0], parse_tree),
            35 => self.factor_1(&children[0], parse_tree),
            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}

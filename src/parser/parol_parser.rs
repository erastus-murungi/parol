// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;
use parol_runtime::lexer::{TokenStream, Tokenizer};
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, UserActionsTrait,
};
use std::cell::RefCell;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 28] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ NEW_LINE_TOKEN,
    /*  2 */ WHITESPACE_TOKEN,
    /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
    /*  4 */ r###"((?ms)/\*.*?\*/)"###,
    /*  5 */ r###"%start"###,
    /*  6 */ r###"%title"###,
    /*  7 */ r###"%comment"###,
    /*  8 */ r###"%line_comment"###,
    /*  9 */ r###"%block_comment"###,
    /* 10 */ r###"%auto_newline_off"###,
    /* 11 */ r###"%auto_ws_off"###,
    /* 12 */ r###"%%"###,
    /* 13 */ r###":"###,
    /* 14 */ r###";"###,
    /* 15 */ r###"\|"###,
    /* 16 */ r###"<"###,
    /* 17 */ r###">"###,
    /* 18 */ r###"\("###,
    /* 19 */ r###"\)"###,
    /* 20 */ r###"\["###,
    /* 21 */ r###"\]"###,
    /* 22 */ r###"\{"###,
    /* 23 */ r###"\}"###,
    /* 24 */ r###"[a-zA-Z_]\w*"###,
    /* 25 */ r###"\u{0022}([^\\]|\\.)*?\u{0022}"###,
    /* 26 */ r###"%scanner"###,
    /* 27 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 28] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "PercentStart",
    /*  6 */ "PercentTitle",
    /*  7 */ "PercentComment",
    /*  8 */ "PercentLineUnderscoreComment",
    /*  9 */ "PercentBlockUnderscoreComment",
    /* 10 */ "PercentAutoUnderscoreNewlineUnderscoreOff",
    /* 11 */ "PercentAutoUnderscoreWsUnderscoreOff",
    /* 12 */ "PercentPercent",
    /* 13 */ "Colon",
    /* 14 */ "Semicolon",
    /* 15 */ "Or",
    /* 16 */ "LT",
    /* 17 */ "GT",
    /* 18 */ "LParen",
    /* 19 */ "RParen",
    /* 20 */ "LBracket",
    /* 21 */ "RBracket",
    /* 22 */ "LBrace",
    /* 23 */ "RBrace",
    /* 24 */ "Identifier",
    /* 25 */ "String",
    /* 26 */ "PercentScanner",
    /* 27 */ "Error",
];

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 34] = &[
    /*  0 */ "Alternation",
    /*  1 */ "AlternationRest",
    /*  2 */ "AlternationRestSuffix",
    /*  3 */ "Alternations",
    /*  4 */ "AlternationsRest",
    /*  5 */ "AlternationsRestSuffix",
    /*  6 */ "AlternationsSuffix",
    /*  7 */ "Declaration",
    /*  8 */ "Factor",
    /*  9 */ "GlobalDirective",
    /* 10 */ "GrammarDefinition",
    /* 11 */ "GrammarDefinitionRest",
    /* 12 */ "GrammarDefinitionRestSuffix",
    /* 13 */ "GrammarDefinitionSuffix",
    /* 14 */ "Group",
    /* 15 */ "Identifier",
    /* 16 */ "Optional",
    /* 17 */ "Parol",
    /* 18 */ "Production",
    /* 19 */ "Prolog",
    /* 20 */ "PrologRest",
    /* 21 */ "PrologRestSuffix",
    /* 22 */ "PrologSuffix",
    /* 23 */ "Repeat",
    /* 24 */ "ScannerDirectives",
    /* 25 */ "ScannerState",
    /* 26 */ "ScannerStateRest",
    /* 27 */ "ScannerStateRestSuffix",
    /* 28 */ "ScannerStateSuffix",
    /* 29 */ "SimpleToken",
    /* 30 */ "StartDeclaration",
    /* 31 */ "String",
    /* 32 */ "Symbol",
    /* 33 */ "TokenWithState",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 34] = &[
    /* 0 - "Alternation" */
    LookaheadDFA {
        states: &[None, Some(30), Some(31)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 16, 1),
            DFATransition(0, 18, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 2),
            DFATransition(0, 22, 1),
            DFATransition(0, 23, 2),
            DFATransition(0, 24, 1),
            DFATransition(0, 25, 1),
        ],
        k: 1,
    },
    /* 1 - "AlternationRest" */
    LookaheadDFA {
        states: &[Some(32)],
        transitions: &[],
        k: 0,
    },
    /* 2 - "AlternationRestSuffix" */
    LookaheadDFA {
        states: &[None, Some(33), Some(34)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 16, 1),
            DFATransition(0, 18, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 2),
            DFATransition(0, 22, 1),
            DFATransition(0, 23, 2),
            DFATransition(0, 24, 1),
            DFATransition(0, 25, 1),
        ],
        k: 1,
    },
    /* 3 - "Alternations" */
    LookaheadDFA {
        states: &[Some(24)],
        transitions: &[],
        k: 0,
    },
    /* 4 - "AlternationsRest" */
    LookaheadDFA {
        states: &[Some(27)],
        transitions: &[],
        k: 0,
    },
    /* 5 - "AlternationsRestSuffix" */
    LookaheadDFA {
        states: &[None, Some(28), Some(29)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 21, 2),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 6 - "AlternationsSuffix" */
    LookaheadDFA {
        states: &[None, Some(25), Some(26)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 21, 2),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 7 - "Declaration" */
    LookaheadDFA {
        states: &[None, Some(10), Some(11), Some(12)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 2),
            DFATransition(0, 8, 3),
            DFATransition(0, 9, 3),
            DFATransition(0, 10, 3),
            DFATransition(0, 11, 3),
        ],
        k: 1,
    },
    /* 8 - "Factor" */
    LookaheadDFA {
        states: &[None, Some(35), Some(36), Some(37), Some(38)],
        transitions: &[
            DFATransition(0, 16, 4),
            DFATransition(0, 18, 1),
            DFATransition(0, 20, 3),
            DFATransition(0, 22, 2),
            DFATransition(0, 24, 4),
            DFATransition(0, 25, 4),
        ],
        k: 1,
    },
    /* 9 - "GlobalDirective" */
    LookaheadDFA {
        states: &[None, Some(7), Some(8)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 1),
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 26, 2),
        ],
        k: 1,
    },
    /* 10 - "GrammarDefinition" */
    LookaheadDFA {
        states: &[Some(17)],
        transitions: &[],
        k: 0,
    },
    /* 11 - "GrammarDefinitionRest" */
    LookaheadDFA {
        states: &[Some(20)],
        transitions: &[],
        k: 0,
    },
    /* 12 - "GrammarDefinitionRestSuffix" */
    LookaheadDFA {
        states: &[None, Some(21), Some(22)],
        transitions: &[DFATransition(0, 0, 2), DFATransition(0, 24, 1)],
        k: 1,
    },
    /* 13 - "GrammarDefinitionSuffix" */
    LookaheadDFA {
        states: &[None, Some(18), Some(19)],
        transitions: &[DFATransition(0, 0, 2), DFATransition(0, 24, 1)],
        k: 1,
    },
    /* 14 - "Group" */
    LookaheadDFA {
        states: &[Some(44)],
        transitions: &[],
        k: 0,
    },
    /* 15 - "Identifier" */
    LookaheadDFA {
        states: &[Some(47)],
        transitions: &[],
        k: 0,
    },
    /* 16 - "Optional" */
    LookaheadDFA {
        states: &[Some(45)],
        transitions: &[],
        k: 0,
    },
    /* 17 - "Parol" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 18 - "Production" */
    LookaheadDFA {
        states: &[Some(23)],
        transitions: &[],
        k: 0,
    },
    /* 19 - "Prolog" */
    LookaheadDFA {
        states: &[Some(1)],
        transitions: &[],
        k: 0,
    },
    /* 20 - "PrologRest" */
    LookaheadDFA {
        states: &[Some(4)],
        transitions: &[],
        k: 0,
    },
    /* 21 - "PrologRestSuffix" */
    LookaheadDFA {
        states: &[None, Some(5), Some(6)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 1),
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 12, 2),
            DFATransition(0, 26, 1),
        ],
        k: 1,
    },
    /* 22 - "PrologSuffix" */
    LookaheadDFA {
        states: &[None, Some(2), Some(3)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 1),
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 12, 2),
            DFATransition(0, 26, 1),
        ],
        k: 1,
    },
    /* 23 - "Repeat" */
    LookaheadDFA {
        states: &[Some(46)],
        transitions: &[],
        k: 0,
    },
    /* 24 - "ScannerDirectives" */
    LookaheadDFA {
        states: &[None, Some(13), Some(14), Some(15), Some(16)],
        transitions: &[
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 3),
            DFATransition(0, 11, 4),
        ],
        k: 1,
    },
    /* 25 - "ScannerState" */
    LookaheadDFA {
        states: &[Some(49)],
        transitions: &[],
        k: 0,
    },
    /* 26 - "ScannerStateRest" */
    LookaheadDFA {
        states: &[Some(52)],
        transitions: &[],
        k: 0,
    },
    /* 27 - "ScannerStateRestSuffix" */
    LookaheadDFA {
        states: &[None, Some(53), Some(54)],
        transitions: &[
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 28 - "ScannerStateSuffix" */
    LookaheadDFA {
        states: &[None, Some(50), Some(51)],
        transitions: &[
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 29 - "SimpleToken" */
    LookaheadDFA {
        states: &[Some(42)],
        transitions: &[],
        k: 0,
    },
    /* 30 - "StartDeclaration" */
    LookaheadDFA {
        states: &[Some(9)],
        transitions: &[],
        k: 0,
    },
    /* 31 - "String" */
    LookaheadDFA {
        states: &[Some(48)],
        transitions: &[],
        k: 0,
    },
    /* 32 - "Symbol" */
    LookaheadDFA {
        states: &[None, Some(39), Some(40), Some(41)],
        transitions: &[
            DFATransition(0, 16, 3),
            DFATransition(0, 24, 1),
            DFATransition(0, 25, 2),
        ],
        k: 1,
    },
    /* 33 - "TokenWithState" */
    LookaheadDFA {
        states: &[Some(43)],
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 55] = &[
    // 0 - Parol: Prolog GrammarDefinition;
    Production {
        lhs: 17,
        production: &[ParseType::N(10), ParseType::N(19)],
    },
    // 1 - Prolog: StartDeclaration PrologSuffix;
    Production {
        lhs: 19,
        production: &[ParseType::N(22), ParseType::N(30)],
    },
    // 2 - PrologSuffix: PrologRest;
    Production {
        lhs: 22,
        production: &[ParseType::N(20)],
    },
    // 3 - PrologSuffix: ;
    Production {
        lhs: 22,
        production: &[],
    },
    // 4 - PrologRest: GlobalDirective PrologRestSuffix;
    Production {
        lhs: 20,
        production: &[ParseType::N(21), ParseType::N(9)],
    },
    // 5 - PrologRestSuffix: PrologRest;
    Production {
        lhs: 21,
        production: &[ParseType::N(20)],
    },
    // 6 - PrologRestSuffix: ;
    Production {
        lhs: 21,
        production: &[],
    },
    // 7 - GlobalDirective: Declaration;
    Production {
        lhs: 9,
        production: &[ParseType::N(7)],
    },
    // 8 - GlobalDirective: ScannerState;
    Production {
        lhs: 9,
        production: &[ParseType::N(25)],
    },
    // 9 - StartDeclaration: "%start" Identifier;
    Production {
        lhs: 30,
        production: &[ParseType::N(15), ParseType::T(5)],
    },
    // 10 - Declaration: "%title" String;
    Production {
        lhs: 7,
        production: &[ParseType::N(31), ParseType::T(6)],
    },
    // 11 - Declaration: "%comment" String;
    Production {
        lhs: 7,
        production: &[ParseType::N(31), ParseType::T(7)],
    },
    // 12 - Declaration: ScannerDirectives;
    Production {
        lhs: 7,
        production: &[ParseType::N(24)],
    },
    // 13 - ScannerDirectives: "%line_comment" String;
    Production {
        lhs: 24,
        production: &[ParseType::N(31), ParseType::T(8)],
    },
    // 14 - ScannerDirectives: "%block_comment" String String;
    Production {
        lhs: 24,
        production: &[ParseType::N(31), ParseType::N(31), ParseType::T(9)],
    },
    // 15 - ScannerDirectives: "%auto_newline_off";
    Production {
        lhs: 24,
        production: &[ParseType::T(10)],
    },
    // 16 - ScannerDirectives: "%auto_ws_off";
    Production {
        lhs: 24,
        production: &[ParseType::T(11)],
    },
    // 17 - GrammarDefinition: "%%" Production GrammarDefinitionSuffix;
    Production {
        lhs: 10,
        production: &[ParseType::N(13), ParseType::N(18), ParseType::T(12)],
    },
    // 18 - GrammarDefinitionSuffix: GrammarDefinitionRest;
    Production {
        lhs: 13,
        production: &[ParseType::N(11)],
    },
    // 19 - GrammarDefinitionSuffix: ;
    Production {
        lhs: 13,
        production: &[],
    },
    // 20 - GrammarDefinitionRest: Production GrammarDefinitionRestSuffix;
    Production {
        lhs: 11,
        production: &[ParseType::N(12), ParseType::N(18)],
    },
    // 21 - GrammarDefinitionRestSuffix: GrammarDefinitionRest;
    Production {
        lhs: 12,
        production: &[ParseType::N(11)],
    },
    // 22 - GrammarDefinitionRestSuffix: ;
    Production {
        lhs: 12,
        production: &[],
    },
    // 23 - Production: Identifier ":" Alternations ";";
    Production {
        lhs: 18,
        production: &[
            ParseType::T(14),
            ParseType::N(3),
            ParseType::T(13),
            ParseType::N(15),
        ],
    },
    // 24 - Alternations: Alternation AlternationsSuffix;
    Production {
        lhs: 3,
        production: &[ParseType::N(6), ParseType::N(0)],
    },
    // 25 - AlternationsSuffix: AlternationsRest;
    Production {
        lhs: 6,
        production: &[ParseType::N(4)],
    },
    // 26 - AlternationsSuffix: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 27 - AlternationsRest: "\|" Alternation AlternationsRestSuffix;
    Production {
        lhs: 4,
        production: &[ParseType::N(5), ParseType::N(0), ParseType::T(15)],
    },
    // 28 - AlternationsRestSuffix: AlternationsRest;
    Production {
        lhs: 5,
        production: &[ParseType::N(4)],
    },
    // 29 - AlternationsRestSuffix: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 30 - Alternation: AlternationRest;
    Production {
        lhs: 0,
        production: &[ParseType::N(1)],
    },
    // 31 - Alternation: ;
    Production {
        lhs: 0,
        production: &[],
    },
    // 32 - AlternationRest: Factor AlternationRestSuffix;
    Production {
        lhs: 1,
        production: &[ParseType::N(2), ParseType::N(8)],
    },
    // 33 - AlternationRestSuffix: AlternationRest;
    Production {
        lhs: 2,
        production: &[ParseType::N(1)],
    },
    // 34 - AlternationRestSuffix: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 35 - Factor: Group;
    Production {
        lhs: 8,
        production: &[ParseType::N(14)],
    },
    // 36 - Factor: Repeat;
    Production {
        lhs: 8,
        production: &[ParseType::N(23)],
    },
    // 37 - Factor: Optional;
    Production {
        lhs: 8,
        production: &[ParseType::N(16)],
    },
    // 38 - Factor: Symbol;
    Production {
        lhs: 8,
        production: &[ParseType::N(32)],
    },
    // 39 - Symbol: Identifier;
    Production {
        lhs: 32,
        production: &[ParseType::N(15)],
    },
    // 40 - Symbol: SimpleToken;
    Production {
        lhs: 32,
        production: &[ParseType::N(29)],
    },
    // 41 - Symbol: TokenWithState;
    Production {
        lhs: 32,
        production: &[ParseType::N(33)],
    },
    // 42 - SimpleToken: String;
    Production {
        lhs: 29,
        production: &[ParseType::N(31)],
    },
    // 43 - TokenWithState: "<" Identifier ">" String;
    Production {
        lhs: 33,
        production: &[
            ParseType::N(31),
            ParseType::T(17),
            ParseType::N(15),
            ParseType::T(16),
        ],
    },
    // 44 - Group: "\(" Alternations "\)";
    Production {
        lhs: 14,
        production: &[ParseType::T(19), ParseType::N(3), ParseType::T(18)],
    },
    // 45 - Optional: "\[" Alternations "\]";
    Production {
        lhs: 16,
        production: &[ParseType::T(21), ParseType::N(3), ParseType::T(20)],
    },
    // 46 - Repeat: "\{" Alternations "\}";
    Production {
        lhs: 23,
        production: &[ParseType::T(23), ParseType::N(3), ParseType::T(22)],
    },
    // 47 - Identifier: "[a-zA-Z_]\w*";
    Production {
        lhs: 15,
        production: &[ParseType::T(24)],
    },
    // 48 - String: "\u{0022}([^\\]|\\.)*?\u{0022}";
    Production {
        lhs: 31,
        production: &[ParseType::T(25)],
    },
    // 49 - ScannerState: "%scanner" Identifier "\{" ScannerStateSuffix;
    Production {
        lhs: 25,
        production: &[
            ParseType::N(28),
            ParseType::T(22),
            ParseType::N(15),
            ParseType::T(26),
        ],
    },
    // 50 - ScannerStateSuffix: ScannerStateRest "\}";
    Production {
        lhs: 28,
        production: &[ParseType::T(23), ParseType::N(26)],
    },
    // 51 - ScannerStateSuffix: "\}";
    Production {
        lhs: 28,
        production: &[ParseType::T(23)],
    },
    // 52 - ScannerStateRest: ScannerDirectives ScannerStateRestSuffix;
    Production {
        lhs: 26,
        production: &[ParseType::N(27), ParseType::N(24)],
    },
    // 53 - ScannerStateRestSuffix: ScannerStateRest;
    Production {
        lhs: 27,
        production: &[ParseType::N(26)],
    },
    // 54 - ScannerStateRestSuffix: ;
    Production {
        lhs: 27,
        production: &[],
    },
];

lazy_static! {
    static ref TOKENIZER: Tokenizer = Tokenizer::build(TERMINALS).unwrap();
}

pub fn parse(
    input: &str,
    file_name: String,
    user_actions: &mut dyn UserActionsTrait,
) -> Result<Tree<ParseTreeType>> {
    let mut llk_parser = LLKParser::new(
        17,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream = RefCell::new(TokenStream::new(input, file_name, &TOKENIZER, MAX_K).unwrap());
    let result = llk_parser.parse(&token_stream, user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}

use regex::Regex;

#[derive(Debug,Clone,PartialEq)]
pub enum Tok {
    PLUS,
    TIMES,
    MINUS,
    DIVIDE,
    LESSTHAN,
    EQUAL,
    NEGATIVE,
    LET,
    SEQUENTIAL,
    ALLOCATE,
    SET,
    GET,
    CONDITIONAL,
    FUN_POINTER,
    CALL,
    FUNCTION,
    PRINT,
    SPAWN,
    I32(i32),
    DOLLAR,
    LPAREN,
    RPAREN,
    ARRAY,
    TYPE_I32,
    RET_TYPE,
    PERCENT,
    VAR_ID(String),
    TRUE,
    FALSE,
    END,
}

#[derive(Debug,Clone)]
pub struct LineInfo {
    pub line_no: u64,
    pub col_no: u64
}

impl LineInfo {
    fn incr_line(&mut self, n: u64) {
        self.col_no = 0;
        self.line_no = self.line_no + n
    }

    fn incr_col(&mut self, n: u64) {
        self.col_no = self.col_no + n
    }
}

macro_rules! lex_upd {
    ( $l:expr, $no_chars:expr, $tok:expr ) => {{
        $l.info.incr_col($no_chars);
        $l.rest = $l.rest.split_at($no_chars).1;
        if $l.comment_depth > 0 { lex($l) }
        else { Ok($tok) }
    }}
}

//returns (rest, tok, tok_num_chars)
fn lex<'a>(l: &mut LexerState<'a>) -> Result<Tok, String> {
    let s = l.rest;

    //Comments
    if s.starts_with("/*") {
        l.comment_depth = l.comment_depth + 1;
        l.rest = s.split_at(2).1;
        lex(l)
    }
    else if s.starts_with("*/") {
        l.comment_depth = l.comment_depth - 1;
        l.rest = s.split_at(2).1;
        lex(l)
    }

    //Whitespace characters
    else if s.starts_with(" ") {
        l.info.incr_col(1);
        l.rest = s.split_at(1).1;
        lex(l)
    }

    else if s.starts_with("\t") {
        l.info.incr_col(1);
        l.rest = s.split_at(1).1;
        lex(l)
    }

    //Newline character sequences
    else if s.starts_with("\r\n") {
        l.info.incr_line(1);
        l.rest = s.split_at(2).1;
        lex(l)
    }
    else if s.starts_with("\r") {
        l.info.incr_line(1);
        l.rest = s.split_at(1).1;
        lex(l)
    }
    else if s.starts_with("\n") {
        l.info.incr_line(1);
        l.rest = s.split_at(1).1;
        lex(l)
    }

    //The rest
    //plus
    else if s.starts_with("+") { lex_upd!(l, 1, Tok::PLUS) }
    //times
    else if s.starts_with("*") { lex_upd!(l, 1, Tok::TIMES) }
    //minus or arrow
    else if s.starts_with("-") {
        if s.contains(">"){
            l.rest = s.split_at(1).1;
            lex_upd!(l, 1, Tok::RET_TYPE)
        }
        else{lex_upd!(l, 1, Tok::MINUS)}
    }
    //divide
    else if s.starts_with("/") { lex_upd!(l, 1, Tok::DIVIDE) }
    //lessthan
    else if s.starts_with("<") { lex_upd!(l, 1, Tok::LESSTHAN) }
    //lparen
    else if s.starts_with("(") { lex_upd!(l, 1, Tok::LPAREN) }
    //rparen
    else if s.starts_with(")") { lex_upd!(l, 1, Tok::RPAREN) }
    //equal
    else if s.starts_with("==") {l.rest = s.split_at(1).1; lex_upd!(l, 1, Tok::EQUAL) }
    //negative
    else if s.starts_with("neg") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::NEGATIVE)
    }
    //let
    else if s.starts_with("let") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::LET)
    }
    //seq
    else if s.starts_with("seq") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::SEQUENTIAL)
    }
    //alloc
    else if s.starts_with("alloc") {
        l.rest = s.split_at(4).1;
        lex_upd!(l, 1, Tok::ALLOCATE)
    }
    //set
    else if s.starts_with("set") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::SET)
    }
    //get
    else if s.starts_with("get") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::GET)
    }
    //conditional
    else if s.starts_with("cond") {
        l.rest = s.split_at(3).1;
        lex_upd!(l, 1, Tok::CONDITIONAL)
    }
    //funpointer
    else if s.starts_with("funptr") {
        l.rest = s.split_at(5).1;
        lex_upd!(l, 1, Tok::FUN_POINTER)
    }
    //percent
    else if s.starts_with("%"){lex_upd!(l, 1, Tok::PERCENT)}
    //call
    else if s.starts_with("call") {
        l.rest = s.split_at(3).1;
        lex_upd!(l, 1, Tok::CALL)
    }
    //function
    else if s.starts_with("fun") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::FUNCTION)
    }
    //print
    else if s.starts_with("print") {
        l.rest = s.split_at(4).1;
        lex_upd!(l, 1, Tok::PRINT)
    }
    //spawn
    else if s.starts_with("spawn") {
        l.rest = s.split_at(4).1;
        lex_upd!(l, 1, Tok::SPAWN)
    }
    //array
    else if s.starts_with("array") {
        l.rest = s.split_at(4).1;
        lex_upd!(l, 1, Tok::ARRAY)
    }
    //i32
    else if s.starts_with("i32") {
        l.rest = s.split_at(2).1;
        lex_upd!(l, 1, Tok::TYPE_I32)
    }
    //true
    else if s.starts_with("true"){
        l.rest = s.split_at(3).1;
        lex_upd!(l, 1, Tok::TRUE)
    }
    //false
    else if s.starts_with("false"){
        l.rest = s.split_at(4).1;
        lex_upd!(l, 1, Tok::FALSE)
    }

    //numbers and variables
    else {
        //variables
        match Regex::new(r"^\A[[:alpha:]][[:alnum:]]*").unwrap().find(s) {
            Some(mat) => {
                assert_eq!(mat.start(), 0);
                let(n, rest) = s.split_at(mat.end());
                l.info.incr_col(mat.end() as u64);
                l.rest = rest;
                if l.comment_depth > 0 {lex(l)}
                else{Ok(Tok::VAR_ID(n.parse::<String>().unwrap()))}
            }
            _ => {
                //numbers
                match Regex::new(r"^\A[[:digit:]]+").unwrap().find(s) {
                    Some(mat) => {
                        assert_eq!(mat.start(), 0);
                        let (n, rest) = s.split_at(mat.end());
                        l.info.incr_col(mat.end() as u64);
                        l.rest = rest;
                        if l.comment_depth > 0 { lex(l) }
                        else { Ok(Tok::I32(n.parse::<i32>().unwrap())) }
                    },
                    None => {
                        //Fall-through cases
                        if s.len() > 0 {
                            if l.comment_depth > 0 {
                                //1. Currently lexing a comment
                                l.info.incr_col(1);
                                l.rest = l.rest.split_at(1).1;
                                lex(l)
                            } else {
                                //2. Otherwise, saw an unexpected token
                                Err(format!("unexpected token '{}'", s.split_at(1).0))
                            }
                        } else {
                            //3. A token was requested but none exists
                            Err(format!("unexpected end of program"))
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct LexerState<'a> {
    comment_depth: u64,
    pub rest: &'a str,
    pub info: LineInfo,
}

impl<'a> LexerState<'a> {
    pub fn new(s: &'a str) -> Self {
        LexerState{
            comment_depth: 0,
            rest: s.trim_end(),
            info: LineInfo{line_no: 1, col_no: 0},
        }
    }

    pub fn peek(self: &mut LexerState<'a>) -> Option<Tok> {
        let revert = self.clone();
        match lex(self) {
            Ok(tok) => {
                *self = revert;
                Some(tok)
            },
            Err(err) => {
                eprintln!("lexer error: {} at {}:{}",
                          err, self.info.line_no, self.info.col_no);
                None
            }
        }
    }

    pub fn next(self: &mut LexerState<'a>) -> Option<Tok> {
        match lex(self) {
            Ok(tok) => Some(tok),
            Err(err) => {
                //eprintln!(r"lexer error: {} at {}:{}",
                         // err, self.info.line_no, self.info.col_no);
                None
            }
        }
    }

    pub fn eat(self: &mut LexerState<'a>, expected: Tok) -> Option<()> {
        if let Some(t) = self.next() {
            if t == expected { Some(()) }
            else { None }}
        else { None }
    }
}

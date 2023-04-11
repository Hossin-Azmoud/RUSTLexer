#[allow(non_snake_case, dead_code)]
use std::fs::File;
use std::io;
use std::io::Read;
use std::env;
use std::fmt;
use std::collections::HashMap;


pub const DQUOTE:   char   = '\"';
pub const SQUOTE:   char   = '\'';
pub const SPACE:     char  = ' ';
pub const NL:        char  = '\n';
pub const OPAR:      char  = '(';
pub const CPAR:      char  = ')';
pub const OCURLY:    char  = '{';
pub const CCURLY:    char  = '}';
pub const PLUS:      char  = '+';
pub const MINUS:     char  = '-';
pub const COMA:      char  = ',';
pub const SEMICOLON: char  = ';';
pub const EQUAL:     char  = '=';
pub const GT:        char  = '>';
pub const LT:        char  = '<';
pub const QM:        char  = '!';
pub const COMMENT:   char  = '/';

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]

enum TokenT {
    // Special tokens.
    COMMENT__,
    DQUOTE__,
    SQUOTE__,
    OPAR__,
    CPAR__,
    OCURLY__,
    CCURLY__,
    PLUS__,
    MINUS__,
    COMA__,
    SEMICOLON__,
    EQUAL__,
    GT__,
    LT__,
    QM__,
    
    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,
}

fn make_token_table() -> HashMap<char, TokenT> {
    let mut map: HashMap<char ,TokenT> = HashMap::new();
    
    //Adding all the keys. 
    map.insert(DQUOTE, TokenT::DQUOTE__);
    map.insert(SQUOTE, TokenT::SQUOTE__);
    map.insert(OPAR, TokenT::OPAR__);
    map.insert(CPAR, TokenT::CPAR__);
    map.insert(OCURLY, TokenT::OCURLY__);
    map.insert(CCURLY, TokenT::CCURLY__);
    map.insert(PLUS, TokenT::PLUS__);
    map.insert(MINUS, TokenT::MINUS__);
    map.insert(COMA, TokenT::COMA__);
    map.insert(SEMICOLON, TokenT::SEMICOLON__);
    map.insert(EQUAL, TokenT::EQUAL__);
    map.insert(GT, TokenT::GT__);
    map.insert(LT, TokenT::LT__);
    map.insert(QM, TokenT::QM__);
    
    // Return the map.
    map
}

impl fmt::Display for TokenT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let printable = match *self {
            TokenT::NONE__      => "NONE__",
            TokenT::DQUOTE__    => "DQUOTE__",
            TokenT::SQUOTE__    => "SQUOTE__",
            TokenT::OPAR__      => "OPAR__",
            TokenT::CPAR__      => "CPAR__",
            TokenT::OCURLY__    => "OCURLY__",
            TokenT::CCURLY__    => "CCURLY__",
            TokenT::PLUS__      => "PLUS__",
            TokenT::MINUS__     => "MINUS__",
            TokenT::COMA__      => "COMA__",
            TokenT::SEMICOLON__ => "SEMICOLON__",
            TokenT::EQUAL__     => "EQUAL__",
            TokenT::GT__        => "GT__",
            TokenT::LT__        => "LT__",
            TokenT::NUMBER__    => "NUMBER__",
            TokenT::STRING__    => "STRING__",
            TokenT::QM__        => "QM__",
            TokenT::VARNAME__   => "VARNAME__",
            TokenT::COMMENT__   => "COMMENT__",
        }; 
       
        write!(f, "{}", printable)
    }
}

struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn empty() -> Self {
        Location {
            row: 0,
            col: 0,
        }
    }

    fn change_loc(&mut self, row: usize, col: usize) {
        // The indexing of raws and cols start from 1, so we need to increment it.
        self.row = row;
        self.col = col;
    }
}

struct Token {
    value:       String,
    token_type:  TokenT,
    size:        usize,
    loc:         Location,
}

impl Token {
    fn empty() -> Self {
        Token {
            value: String::from(""),
            token_type: TokenT::NONE__,
            size: 0,
            loc: Location::empty(),
        }
    }
    
    
    fn write(&mut self, c: char) {
        self.value += &String::from(c);
        self.size += 1;
    }
    
    fn display_token(&mut self, file: &str) {
        println!();  
        
        println!("r: {}", self.loc.row);
        println!("c: {}", self.loc.col);
        println!("t: {}", self.token_type);
        println!("v: {}", self.value);

        println!();  
    }
}

struct Lexer<'a> {
    file_path: &'a str,
    source:    Vec<u8>,
    cur:       usize,
    row:       usize,
    col:       usize,
    size:      usize,
    token_table: HashMap<char, TokenT>,
} 


impl<'a> Lexer<'a> {
    
    fn new(path: &'a str) -> Self    
    {
        return Lexer {
            file_path: path,
            source:    vec![],
            cur:       0,
            row:       1,
            col:       1,
            size:      0,
            token_table: make_token_table(),
        };
    }


    fn get_char(&mut self, index: usize) -> char {   
        if self.is_not_empty() {
            return char::from(self.source[index]);
        }
        
        return '\0';
    }

    fn get_current(&mut self) -> char {
        return self.get_char(self.cur);
    }

    fn is_not_empty(&mut self) -> bool {
        return self.cur < self.size;
    }
    
    fn display(&mut self) {     
        println!("fp: {}", self.file_path);
        println!("sourcelen: {}", self.source.len());
        println!("cur: {}", self.cur);
        println!("row: {}", self.row);
        println!("col: {}", self.col);
    }

    fn chop(&mut self) -> usize {
        if self.is_not_empty()  {
            self.cur += 1;
    
            let c: char = self.get_current();
                        
            if c == NL {
                self.row  += 1;
                self.col =  0;
                 
                return self.cur;
            }
            
            self.col += 1;
        }

        return self.cur;
    }
        
    fn read(&mut self) -> io::Result<()> {
        let mut tmp = File::open(self.file_path)?; 
        tmp.read_to_end(&mut self.source)?;
        self.size = self.source.len();
        
        Ok(())
    }

    fn match_current(&mut self, token: &mut Token) -> Result<(), io::Error> {
        let mut c: char = self.get_current();
        // it is a known token.
        token.loc.change_loc(self.row, self.col);
        
        if self.token_table.contains_key(&c) {
            token.write(c);
            token.token_type = self.token_table[&c];
            self.chop();
            return Ok(());
        }

        if c.is_ascii_punctuation(){
            if c == COMMENT {
                
                token.token_type = TokenT::COMMENT__;
                token.write(c);
                
                self.chop();
                c = self.get_current();
                
                if c != COMMENT {
                    let err_text = format!("Expected a / in Line {}: column: {} found: |{}|", self.row, self.col, c);
                    let err = io::Error::new(io::ErrorKind::Other, err_text);
                    return Err(err);
                }
                // Write the second char /
                while c != NL {
                    token.write(c);
                    self.chop();
                    c = self.get_current();
                }
                

                return Ok(());
            };
            
            token.write(c);
            token.token_type = TokenT::NONE__;
            self.chop();
        }

        return Ok(());
    }
    
    fn trim_spaces_left(&mut self)  {
        while self.get_current().is_ascii_whitespace()
        {
            self.chop();
        }
    }


    fn collect_str(&mut self, token: &mut Token){
        token.token_type = TokenT::STRING__;
        let mut c: char = self.get_current();
        
        while c.is_alphanumeric() || c.is_digit(10) && self.is_not_empty() {
            if c.is_ascii_punctuation() {
                break;
            }
            
            if c.is_ascii_whitespace() {
                break;
            }

            token.write(c);
            self.chop();

            c = self.get_current();
        }
    }
    
    fn collect_number(&mut self, token: &mut Token) {
        
        token.token_type = TokenT::NUMBER__;
        let mut c: char = self.get_current();

        while self.is_not_empty() && c.is_digit(10) {
            
            if c.is_ascii_punctuation() {
                break;
            }

            if c.is_ascii_whitespace() {
                break;
            }
            
            token.write(c);
            self.chop();
            c = self.get_current();
        }
    }

    fn next(&mut self) -> Result<Token, io::Error> {
        
        self.trim_spaces_left();
        let mut token = Token::empty();
        
        // TODO: Match with already defined tokens.
        let res = self.match_current(&mut token);
         
        match res {
            Err(e) => return Err(e),
            Ok(())   => {
                if token.size > 0 { 
                    return Ok(token);
                }
                 
                let c: char = self.get_current(); 
                 
                token.loc.change_loc(self.row, self.col);
                if c.is_alphanumeric() {
                    self.collect_str(&mut token);
                }

                if c.is_digit(10) {            
                    self.collect_number(&mut token);            
                }

                return Ok(token);
            }
        }
    }
}

#[warn(unused_variables)]
fn main() -> Result<(), io::Error>
{
    // Command line args
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    
    if args.len() < 2 {
        
        println!("---------------------------------");
        println!("The File path was not provided.");
        println!("Usage: {} <path>", program);
        println!("---------------------------------");
        return Ok(());
    }
    
    let src = &args[1];
    
    let mut lex: Lexer = Lexer::new(&src);

    lex.display();
    lex.read()?;
    
    let mut t;
    
   
    println!("------------------ LEXER -------------------");
    
    while lex.is_not_empty() {
        t = lex.next();
        match t {
            Ok(mut token) => token.display_token(&src),
            Err(e) => {
                println!("ERROR: {}", e);
                return Ok(());
            },
        }
    }

    println!("--------------------------------------");
    
    return Ok(());
}


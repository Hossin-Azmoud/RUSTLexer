use std::fs::File;
use std::io;
use std::io::Read;

// TODO: Read file.
// TODO: tokenize file content
struct Token {
    token_type:  String,
    value: String,
    size:  usize,
}

impl Token {
    fn empty() -> Self {
        Token {
            token_type: String::from(""),
            value: String::from(""),
            size: 0,
        }
    }
    
    fn new(t: &String, v: &String)  -> Self {
        Token {
            token_type: t.to_string(),
            value: v.to_string(),
            size: v.len(),
        }
    }

    fn write(&mut self, c: char) -> String {
        self.value += &String::from(c);
        self.size += 1;
        return self.value.clone();
    }
}

struct Lexer<'a> {
    file_path: &'a str,
    source:    Vec<u8>,
    cur:       usize,
    row:       usize,
    col:       usize,
    size:      usize,
} 

/*
0 => func
1 => =>
2 => (
3 => (
4 => x
5 => ,
6 => y
7 => )
8 => =>
9 => x
10 => +
11 => y
12 => )
13 => write
14 => (
15 => func
16 => (
17 => 1
18 => ,
19 => 2
20 => )
21 => )
22 => 1
23 => >=
24 => 2
*/


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

impl<'a> Lexer<'a> {
    
    fn new(path: &'a str) -> Self    
    {
        return Lexer {
            file_path: path,
            source:    vec![],
            
            cur:       0,
            row:       0,
            col:       0,
            size:      0,
        };
    }


    fn get_char(&mut self, index: usize) -> char {   
        return char::from(self.source[index]);
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
                self.row += 1;
                self.col = 0;
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
    
    fn match_current(&mut self, token: &mut Token)  {
        let mut c: char = self.get_current();
        println!("Matching..");
        match c {
            MINUS => {
                token.write(MINUS);
                self.chop();
                
            },
            PLUS => {
                token.write(PLUS);
                self.chop();
                
            },
            OPAR =>  {
                token.write(OPAR);
                self.chop();
                
            },
            CPAR =>  {
                token.write(CPAR);
                self.chop();
                
            },

            OCURLY => {
                token.write(OCURLY);
                self.chop();
                
            },
            
            CCURLY => {
                token.write(CCURLY);
                self.chop();
                
            },
            COMA => {
                token.write(COMA);
                self.chop();
                
            },
            LT => {
                    token.write(LT);
                    self.chop();
                    if self.is_not_empty() {
                        c = self.get_current();
                        // =>, <=, >=?
                        if c == EQUAL {
                            token.write(EQUAL);
                            self.chop();
                        }
                    }
                    
                },
                GT => {
                    token.write(GT);
                    self.chop();
                    if self.is_not_empty() {
                        c = self.get_current();
                        // =>, <=, >=?
                        if c == EQUAL {
                            token.write(EQUAL);
                            self.chop();
                        }
                    }
                    
                },
                EQUAL => {
                    token.write(EQUAL);
                    self.chop();
                    
                    if self.is_not_empty() {
                        c = self.get_current();
                        // =>, <=, >=?
                        if c == GT {
                            token.write(GT);
                        }
                        self.chop();
                    } 
                },
                _ => {
                    return;
                },
        }
    }
    
    fn trim_spaces_left(&mut self)  {
        while self.get_current().is_ascii_whitespace()
        {
            self.chop();
        }
    }

    fn next(&mut self) -> Token {
        self.trim_spaces_left();
        let mut token = Token::empty();
        
        
        // TODO: Match with already defined tokens.
        self.match_current(&mut token);
            
        if token.size > 0 {
            return token;
        }
        
        let mut c: char = self.get_current(); 
        if c.is_alphanumeric() {
            while c.is_alphanumeric() || c.is_digit(10) && self.is_not_empty() {
                // println!("ALPHA_CONSUME");
                token.write(c);      
                self.chop();
                c = self.get_current();
            }
        }

        if c.is_digit(10) {
            while c.is_digit(10) && self.is_not_empty() {
                // println!("ALPHA_CONSUME");
                token.write(c);
                self.chop();
                c = self.get_current();
            }
        }

        while self.is_not_empty() && !c.is_ascii_whitespace() {
            // TODO: Match unknown tokens. 
            token.write(c);
            c = self.get_current();
        }

        return token;
    }
}

#[warn(unused_variables)]
fn main() -> io::Result<()>
{
    let mut lex: Lexer = Lexer::new("source");
    
    lex.display();
    lex.read()?;
    
    let t: Token = lex.next();
    println!("{}", t.value);
    
    Ok(())
}

/*
#[warn(dead_code)]
fn main1() -> io::Result<()>{
    
    const SPACE:     char  = ' ';
    const NL:        char  = '\n';
    
    const OPAR:      char  = '(';
    const CPAR:      char  = ')';
    const OCURLY:    char  = '{';
    const CCURLY:    char  = '}';
    const PLUS:      char  = '+';
    const MINUS:     char  = '-';
    const COMA:      char  = ',';
    const SEMICOLON: char  = ';';
    const EQUAL:     char  = '=';
    const GT:        char  = '>';
    const LT:        char  = '<';

    let mut source = File::open("source")?;
    let mut contents = vec![];
    
    source.read_to_end(&mut contents)?;

    let mut index:        usize  = 0;
    let mut token_index:  usize  = 0;
    let mut token_buffer: String = String::new();
    let length: usize            = contents.len();
    
    while index < length {

        let mut c: char = char::from(contents[index]);

        while !c.is_ascii_whitespace() && index < length
        {
            match c {
                MINUS => {
                    token_buffer = String::from(MINUS);
                    index += 1;
                    break;
                },
                LT => {
                    token_buffer = String::from(LT);
                    index += 1;
                    if index < length {
                        c = char::from(contents[index]);
                        // =>, <=, >=?
                        if c == EQUAL {
                            token_buffer += &String::from(EQUAL);
                            index += 1;
                        }
                    }
                    break;
                },
                GT => {
                    token_buffer = String::from(GT);
                    index += 1;
                    if index < length {
                        c = char::from(contents[index]);
                        // =>, <=, >=?
                        if c == EQUAL {
                            token_buffer += &String::from(EQUAL);
                            index += 1;
                        }
                    }
                    break;
                },

                SPACE => {
                    index += 1;
                    break;
                },
                EQUAL => {
                    token_buffer = String::from(EQUAL);
                    index += 1;
                    
                    if index < length {
                        c = char::from(contents[index]);
                        // =>, <=, >=?
                        if c == GT {
                            token_buffer += &String::from(GT);
                        }
                        index += 1;
                    }
                    break;
                },
                OPAR =>  {
                    token_buffer = String::from(OPAR);
                    index += 1;
                    break;
                },
                CPAR =>  {
                    token_buffer = String::from(CPAR);
                    index += 1;
                    break;
                },

                OCURLY => {
                    token_buffer = String::from(OCURLY);
                    index += 1;
                    break;
                },
                
                CCURLY => {
                    token_buffer = String::from(CCURLY);
                    index += 1;
                    break;
                },
                COMA => {
                    token_buffer = String::from(COMA);
                    index += 1;
                    break;
                },
                SEMICOLON => {
                    token_buffer = String::from(SEMICOLON);
                    index += 1;
                    break;
                },
                
                PLUS => {
                    token_buffer = String::from(PLUS);
                    index += 1;
                    break;
                },

                _ => {
                    if c.is_alphanumeric() {
                        while c.is_alphanumeric() || c.is_digit(10) {
                            // println!("ALPHA_CONSUME");
                            if c == SPACE {
                                break;
                            }
                            token_buffer += &String::from(c);
                                  
                            if index < contents.len() { index += 1; } else {
                                break;
                            }
                            
                            c = char::from(contents[index]);
                        }
                    } else if c.is_digit(10) {
                        
                        while c.is_digit(10) {
                            //println!("DIGIT_CONSUME");
                            if c == SPACE {                
                                break;
                            }
                        
                            token_buffer += &String::from(c);
                                  
                            if index < contents.len() { index += 1; } else {
                                break;
                            }
                            
                            c = char::from(contents[index]);
                        }
                        
                    } else {
                        token_buffer = String::from(c);
                    }
                    
                    break;
                }
            }
        }
        
        /*        
        
        if token_buffer.len() > 0 {
            print!("{} => { }\n", token_index, token_buffer);
            token_buffer = String::new();
        }
        
        */
        
        if token_buffer.len() > 0 {
            print!("{} => { }\n", token_index, token_buffer);
            token_buffer = String::new();
            token_index += 1;
        }

        if c.is_ascii_whitespace() {
            index += 1; 
        }
        
    }
    
 
    Ok(())

}
*/




/*
struct token {
    Type:  String,
    Value: String,
};

struct lexer {
    file_path: &str,
    source: String,
    cur: i32,
    row: i32,
    col: i32,
};

impl lexer {

    fn new(file_path: &str) -> self {
        lexer {
            file_path: file_path,
            source: String::new(),
            cur: 0,
            row: 0,
            col: 0,
        };   
    }
    
    fn next(&mut self) -> token {
        return 
    }
    
    fn start(&mut self) {
        let mut f = File::open(self.file_path)?;
        let mut buffer = ;
        f.read_to_string(&mut self.source)?;
    }

    fn is_empty(&mut self) {
        
    }
}
*/


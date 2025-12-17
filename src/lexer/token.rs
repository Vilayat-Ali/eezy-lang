pub mod keyword {
    pub const FUNC: &str = "func";
    pub const VAR: &str = "var";
    pub const PUB: &str = "pub";
    pub const IF: &str = "if";
    pub const ELIF: &str = "elif";
    pub const ELSE: &str = "else";
    pub const FOR: &str = "for";
    pub const WHILE: &str = "while";
    pub const IMPORT: &str = "@import";
    pub const CLASS: &str = "class";
    pub const CONSTRUCTOR: &str = "constructor";
    pub const ENUM: &str = "enum";
    pub const RETURN: &str = "return";
}

pub mod punct {
    pub const COLON: &str = ":";
    pub const SEMICOLON: &str = ";";
    pub const COMMA: &str = ",";
    pub const DOT: &str = ".";
    pub const DOUBLE_COLON: &str = "::";
    pub const ARROW: &str = "->";
    pub const FAT_ARROW: &str = "=>";

    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";
    pub const LBRACKET: &str = "[";
    pub const RBRACKET: &str = "]";

    pub const EQ: &str = "=";
    pub const EQ_EQ: &str = "==";
    pub const NOT_EQ: &str = "!=";
    pub const LT: &str = "<";
    pub const GT: &str = ">";
    pub const LTE: &str = "<=";
    pub const GTE: &str = ">=";

    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const STAR: &str = "*";
    pub const SLASH: &str = "/";
    pub const PERCENT: &str = "%";

    pub const BANG: &str = "!";
    pub const AMPERSAND: &str = "&";
    pub const PIPE: &str = "|";
    pub const CARET: &str = "^";
    pub const TILDE: &str = "~";

    pub const PLUS_EQ: &str = "+=";
    pub const MINUS_EQ: &str = "-=";
    pub const STAR_EQ: &str = "*=";
    pub const SLASH_EQ: &str = "/=";
    pub const PERCENT_EQ: &str = "%=";

    pub const AMPERSAND_AMPERSAND: &str = "&&";
    pub const PIPE_PIPE: &str = "||";
}

pub mod ty {
    pub const INT: &str = "int";
    pub const FLOAT: &str = "float";
    pub const DOUBLE: &str = "double";
    pub const BOOL: &str = "bool";
    pub const STRING: &str = "string";
    pub const CHAR: &str = "char";
    pub const VOID: &str = "void";
    pub const ANY: &str = "any";
}

pub mod literal {
    pub const INTEGER: &str = "INTEGER_LITERAL";
    pub const FLOAT: &str = "FLOAT_LITERAL";
    pub const STRING: &str = "STRING_LITERAL";
    pub const CHAR: &str = "CHAR_LITERAL";
    pub const BOOLEAN: &str = "BOOLEAN_LITERAL";
}

pub mod special {
    pub const IDENTIFIER: &str = "IDENTIFIER";
    pub const COMMENT: &str = "COMMENT";
    pub const WHITESPACE: &str = "WHITESPACE";
    pub const NEWLINE: &str = "NEWLINE";
    pub const EOF: &str = "EOF";
    pub const DIRECTIVE: &str = "DIRECTIVE";
}

pub mod stdlib {
    pub const STD: &str = "std";
    pub const PRINTLN: &str = "println";
    pub const PRINT: &str = "print";
    pub const READLINE: &str = "readline";
    pub const LENGTH: &str = "length";
    pub const PUSH: &str = "push";
    pub const POP: &str = "pop";
    pub const TO_STRING: &str = "to_string";
    pub const PARSE_INT: &str = "parse_int";
    pub const PARSE_FLOAT: &str = "parse_float";
}

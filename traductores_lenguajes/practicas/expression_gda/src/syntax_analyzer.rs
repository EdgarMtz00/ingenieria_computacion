use crate::symbols::Token;
use crate::gda::Node;

//Recursive Predictive Descent Parser from suffix math expression to prefix math expression
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.first()
    }

    fn consume(&mut self, token: Token) {
        if self.tokens[0] == token {
            self.tokens.remove(0);
        } else {
            panic!("Unexpected token: {:?}", token);
        }
    }

    // expr -> term expr_tail
    pub fn expression(&mut self) -> Node {
        let term = self.term();
        self.expression_tail(term)
    }

    // expr_tail -> + term expr_tail
    //            | - term expr_tail
    //            | ε
    fn expression_tail(&mut self, left: Node) -> Node {
        match self.peek() {
            Some(Token::Plus) => {
                self.consume(Token::Plus);
                let right = self.term();
                let mut node = Node::new(Token::Plus);
                node.add_child(left);
                node.add_child(self.expression_tail(right));
                self.expression_tail(node)
            }
            Some(Token::Minus) => {
                self.consume(Token::Minus);
                let right = self.term();
                let mut node = Node::new(Token::Minus);
                node.add_child(left);
                node.add_child(self.expression_tail(right));
                self.expression_tail(node)
            }
            _ => left,
        }
    }

    // term -> factor term_tail
    fn term(&mut self) -> Node {
        let factor = self.factor();
        self.term_tail(factor)
    }

    // term_tail -> * factor term_tail
    //            | / factor term_tail
    //            | ε
    fn term_tail(&mut self, left: Node) -> Node {
        match self.peek() {
            Some(Token::Multiply) => {
                self.consume(Token::Multiply);
                let right = self.factor();
                let mut node = Node::new(Token::Multiply);
                node.add_child(left);
                node.add_child(self.term_tail(right));
                self.term_tail(node)
            }
            Some(Token::Divide) => {
                self.consume(Token::Divide);
                let right = self.factor();
                let mut node = Node::new(Token::Divide);
                node.add_child(left);
                node.add_child(self.term_tail(right));
                self.term_tail(node)
            }
            _ => left,
        }
    }

    // factor -> ( expr )
    //         | number
    //         | identifier
    fn factor(&mut self) -> Node {
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.consume(Token::Number(n));
                Node::new(Token::Number(n))
            },
            Some(Token::Identifier(s)) => {
                let id = s.clone();
                self.consume(Token::Identifier(id.clone()));
                Node::new(Token::Identifier(id))
            },
            Some(Token::LeftParen) => {
                self.consume(Token::LeftParen);
                let e = self.expression();
                self.consume(Token::RightParen);
                e
            }
            _ => panic!("Unexpected token"),
        }
    }
}
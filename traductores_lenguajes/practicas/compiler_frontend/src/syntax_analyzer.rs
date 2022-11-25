use crate::symbols::{Operation, SymbolTable, Token};
use crate::three_address_code::{condition_quadruple, Quadruple, while_loop_quadruple};

pub struct Parser {
    tokens: Vec<Token>,
    symbol_table: SymbolTable,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            symbol_table: SymbolTable::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.first()
    }

    fn consume(&mut self, token: Token) {
        if self.tokens[0] == token {
            self.tokens.remove(0);
        } else {
            panic!("Unexpected token! Expected: {:?}, received {:?} ", token, self.tokens[0]);
        }
    }

    // program -> begin declarations orders end
    pub fn program(&mut self) -> Vec<Quadruple> {
        self.consume(Token::Begin);
        self.declarations();
        let orders = self.orders();
        self.consume(Token::End);

        orders
    }

    // declarations -> declaration semicolon next_declarations
    pub fn declarations(&mut self) {
        self.declaration();
        self.consume(Token::Semicolon);
        self.next_declarations();
    }

    // next_declarations -> declaration semicolon next_declarations
    //                    | ε
    pub fn next_declarations(&mut self) {
        match self.peek() {
            Some(Token::IntegerType) => {
                self.declaration();
                self.consume(Token::Semicolon);
                self.next_declarations();
            }
            Some(Token::RealType) => {
                self.declaration();
                self.consume(Token::Semicolon);
                self.next_declarations();
            }
            _ => (),
        }
    }

    // declaration -> type var_list
    pub fn declaration(&mut self) {
        let var_type = self.var_type();
        let var_list = self.var_list();

        for var in var_list {
            self.symbol_table.add_symbol(var, var_type.clone());
        }
    }

    // var_type -> integer | real
    pub fn var_type(&mut self) -> String {
        match self.peek() {
            Some(Token::IntegerType) => {
                self.consume(Token::IntegerType);
                return "integer".to_string();
            }
            Some(Token::RealType) => {
                self.consume(Token::RealType);
                return "real".to_string();
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    // var_list -> identifier next_var_list
    pub fn var_list(&mut self) -> Vec<String> {
        let mut var_list = Vec::new();
        match self.peek() {
            Some(Token::Identifier(_)) => {
                let identifier = self.identifier();
                var_list.push(identifier);
                var_list.append(&mut self.next_var_list());
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
        var_list
    }

    // next_var_list -> comma identifier next_var_list
    //                | ε
    pub fn next_var_list(&mut self) -> Vec<String> {
        let mut var_list = Vec::new();
        match self.peek() {
            Some(Token::Comma) => {
                self.consume(Token::Comma);
                let identifier = self.identifier();
                var_list.push(identifier);
                var_list.append(&mut self.next_var_list());
            }
            _ => (),
        }
        var_list
    }

    // orders -> order semicolon next_orders
    pub fn orders(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();
        let mut order = self.order().unwrap_or_else(|| panic!("Unexpected token: {:?}", self.peek()));
        result.append(&mut order);
        self.consume(Token::Semicolon);
        result.append(&mut self.next_orders());
        result
    }

    // next_orders -> order semicolon next_orders
    //              | ε
    pub fn next_orders(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();
        if let Some(mut order) = self.order() {
            result.append(&mut order);
            if let Some(Token::Semicolon) = self.peek() {
                self.consume(Token::Semicolon);
            }
            result.append(&mut self.next_orders());
        }
        result
    }

    // order -> condition | while_loop | assign
    pub fn order(&mut self) -> Option<Vec<Quadruple>> {
        match self.peek() {
            Some(Token::If) => Some(self.condition()),
            Some(Token::While) => Some(self.while_loop()),
            Some(Token::Identifier(_)) => Some(self.assign()),
            _ => None,
        }
    }

    // condition -> if ( comparison ) orders next_condition
    pub fn condition(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();

        self.consume(Token::If);
        self.consume(Token::LeftParenthesis);

        let comparison = self.comparison();
        let comparison_result = comparison.result.clone();

        self.consume(Token::RightParenthesis);

        result.push(comparison);

        let orders = self.orders();
        result.append(&mut condition_quadruple(comparison_result.clone(), orders, false));

        result.append(&mut self.next_condition(comparison_result));
        result
    }

    // next_condition -> end
    //                 | else orders end
    pub fn next_condition(&mut self, comparison: String) -> Vec<Quadruple> {
        match self.peek() {
            Some(Token::End) => {
                self.consume(Token::End);
                Vec::new()
            }
            Some(Token::Else) => {
                self.consume(Token::Else);
                let orders = self.orders();
                self.consume(Token::End);
                condition_quadruple(comparison, orders, true)
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    // comparison -> operator condition_operator operator
    pub fn comparison(&mut self) -> Quadruple {
        let operator1 = self.operator();
        let condition_operation = self.condition_operation();
        let operator2 = self.operator();

        Quadruple {
            op: condition_operation,
            arg1: Some(operator1),
            arg2: Some(operator2),
            result: self.symbol_table.new_temp("boolean".to_string()),
        }
    }

    // condition_operator -> == | != | < | > | <= | >=
    pub fn condition_operation(&mut self) -> Operation {
        match self.peek() {
            Some(Token::Equal) => {
                self.consume(Token::Equal);
                Operation::Equal
            }
            Some(Token::NotEqual) => {
                self.consume(Token::NotEqual);
                Operation::NotEqual
            }
            Some(Token::LessThan) => {
                self.consume(Token::LessThan);
                Operation::LessThan
            }
            Some(Token::GreaterThan) => {
                self.consume(Token::GreaterThan);
                Operation::GreaterThan
            }
            Some(Token::LessThanEqual) => {
                self.consume(Token::LessThanEqual);
                Operation::LessThanEqual
            }
            Some(Token::GreaterThanEqual) => {
                self.consume(Token::GreaterThanEqual);
                Operation::GreaterThanEqual
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    // operator -> identifier | number
    pub fn operator(&mut self) -> String {
        match self.peek() {
            Some(Token::Identifier(_)) => self.identifier(),
            Some(Token::Real(_)) => self.number(),
            Some(Token::Integer(_)) => self.number(),
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    pub fn number(&mut self) -> String {
        match self.peek() {
            Some(Token::Real(number)) => {
                let n = number.clone();
                self.consume(Token::Real(*number));
                n.to_string()
            }
            Some(Token::Integer(number)) => {
                let n = number.clone();
                self.consume(Token::Integer(*number));
                n.to_string()
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    // while_loop -> while ( comparison ) orders endwhile
    pub fn while_loop(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();
        self.consume(Token::While);
        self.consume(Token::LeftParenthesis);

        let comparison = self.comparison();
        let comparison_result = comparison.result.clone();

        result.push(comparison);

        self.consume(Token::RightParenthesis);

        let orders = self.orders();

        self.consume(Token::EndWhile);

        result.append(&mut while_loop_quadruple(comparison_result, orders));
        result
    }

    // assign -> identifier := expression
    pub fn assign(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();

        let identifier = self.identifier();
        self.consume(Token::Assign);
        let mut expression = self.expression();


        let last_result = expression.last().unwrap().result.clone();
        result.append(&mut expression);
        result.push(Quadruple {
            op: Operation::Assign,
            arg1: Some(last_result),
            arg2: None,
            result: identifier,
        });
        result
    }

    // expression -> ( expression arithmetic_operator expression ) next_expression
    //             | identifier next_expression
    //             | number next_expression
    pub fn expression(&mut self) -> Vec<Quadruple> {
        let mut result = Vec::new();
        match self.peek() {
            Some(Token::LeftParenthesis) => {
                self.consume(Token::LeftParenthesis);

                let mut expression1 = self.expression();
                result.append(&mut expression1);

                let arithmetic_operator = self.arithmetic_operator().unwrap_or_else(||{
                        panic!("Unexpected token: {:?}", self.peek())
                    });

                let mut expression2 = self.expression();
                result.append(&mut expression2);

                self.consume(Token::RightParenthesis);

                let expression_variable = self.symbol_table.new_temp("real".to_string());

                result.push(Quadruple {
                    op: arithmetic_operator,
                    arg1: Some(expression1.last().unwrap().result.clone()),
                    arg2: Some(expression2.last().unwrap().result.clone()),
                    result: expression_variable.clone(),
                });

                result.append(&mut self.next_expression(expression_variable));
            }
            Some(Token::Identifier(_)) => {
                let identifier = self.identifier();
                result.append(&mut self.next_expression(identifier));
            }
            Some(Token::Real(number)) => {
                let n = number.to_string().clone();
                self.consume(Token::Real(*number));
                let number_variable = self.symbol_table.new_temp("real".to_string());
                result.push(Quadruple {
                    op: Operation::Assign,
                    arg1: Some(n),
                    arg2: None,
                    result: number_variable.clone(),
                });
                result.append(&mut self.next_expression(number_variable));
            }
            Some(Token::Integer(number)) => {
                let n = number.to_string().clone();
                self.consume(Token::Integer(*number));
                let number_variable = self.symbol_table.new_temp("integer".to_string());
                result.push(Quadruple {
                    op: Operation::Assign,
                    arg1: Some(n),
                    arg2: None,
                    result: number_variable.clone(),
                });
                result.append(&mut self.next_expression(number_variable));
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
        result
    }

    // next_expression -> arithmetic_operator expression next_expression
    //              | ε
    pub fn next_expression(&mut self, first_expression: String) -> Vec<Quadruple> {
        let mut result = Vec::new();

        if let Some(arithmetic_operator) = self.arithmetic_operator(){
            let mut expression = self.expression();
            let expression_variable = expression.last().unwrap().result.clone();
            result.append(&mut expression);
            let new_expression = self.symbol_table.new_temp("real".to_string());
            result.push(Quadruple {
                op: arithmetic_operator,
                arg1: Some(first_expression),
                arg2: Some(expression_variable),
                result: new_expression.clone(),
            });
            result.append(&mut self.next_expression(new_expression));
        }

        result
    }

    // arithmetic_operator -> + | - | * | /
    pub fn arithmetic_operator(&mut self) -> Option<Operation> {
        match self.peek() {
            Some(Token::Plus) => {
                self.consume(Token::Plus);
                Some(Operation::Plus)
            }
            Some(Token::Minus) => {
                self.consume(Token::Minus);
                Some(Operation::Minus)
            }
            Some(Token::Multiply) => {
                self.consume(Token::Multiply);
                Some(Operation::Multiply)
            }
            Some(Token::Divide) => {
                self.consume(Token::Divide);
                Some(Operation::Divide)
            }
            _ => None,
        }
    }

    fn identifier(&mut self) -> String {
        match self.peek() {
            Some(Token::Identifier(identifier)) => {
                let result = identifier.clone();
                self.consume(Token::Identifier(identifier.to_string()));
                return result;
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }
}
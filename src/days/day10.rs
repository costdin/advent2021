use super::get_input_lines;

pub fn day10() {
    let input = get_input_lines(10)
        .iter()
        .map(|s| s.chars().filter_map(Token::parse).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (result1, mut incompletes) =
        input
            .iter()
            .fold((0, vec![]), |(mut invalid, mut incompletes), line| {
                match process_line(line) {
                    LineProcessResult::Incomplete(v) => {
                        incompletes.push(v);
                    }
                    LineProcessResult::Invalid(t) => {
                        invalid += t;
                    }
                }

                (invalid, incompletes)
            });

    incompletes.sort();
    let result2 = incompletes[incompletes.len() / 2];

    println!("DAY 10\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn process_line(line: &[Token]) -> LineProcessResult {
    let mut stack = vec![];

    for t in line {
        if t.is_close() {
            if !t.closes(stack.pop().unwrap()) {
                return LineProcessResult::Invalid(t.invalid_score());
            }
        } else {
            stack.push(t);
        }
    }

    let v = stack
        .iter()
        .rev()
        .filter_map(|v| v.closed_by())
        .fold(0u64, |acc, v| acc * 5 + v.incomplete_score());

    return LineProcessResult::Incomplete(v);
}

enum LineProcessResult {
    Invalid(u64),
    Incomplete(u64),
}

enum Token {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    TriangleOpen,
    TriangleClose,
}

impl Token {
    fn parse(s: char) -> Option<Token> {
        match s {
            '(' => Some(Token::RoundOpen),
            ')' => Some(Token::RoundClose),
            '[' => Some(Token::SquareOpen),
            ']' => Some(Token::SquareClose),
            '{' => Some(Token::CurlyOpen),
            '}' => Some(Token::CurlyClose),
            '<' => Some(Token::TriangleOpen),
            '>' => Some(Token::TriangleClose),
            _ => None,
        }
    }

    fn closes(&self, open_token: &Token) -> bool {
        match (self, open_token) {
            (Token::RoundClose, Token::RoundOpen) => true,
            (Token::SquareClose, Token::SquareOpen) => true,
            (Token::CurlyClose, Token::CurlyOpen) => true,
            (Token::TriangleClose, Token::TriangleOpen) => true,
            _ => false,
        }
    }

    fn is_close(&self) -> bool {
        match self {
            Token::RoundClose | Token::SquareClose | Token::CurlyClose | Token::TriangleClose => {
                true
            }
            _ => false,
        }
    }

    fn invalid_score(&self) -> u64 {
        match self {
            Token::RoundClose => 3,
            Token::SquareClose => 57,
            Token::CurlyClose => 1197,
            Token::TriangleClose => 25137,
            _ => 0,
        }
    }

    fn incomplete_score(&self) -> u64 {
        match self {
            Token::RoundClose => 1,
            Token::SquareClose => 2,
            Token::CurlyClose => 3,
            Token::TriangleClose => 4,
            _ => 0,
        }
    }

    fn closed_by(&self) -> Option<Token> {
        match self {
            Token::RoundOpen => Some(Token::RoundClose),
            Token::SquareOpen => Some(Token::SquareClose),
            Token::CurlyOpen => Some(Token::CurlyClose),
            Token::TriangleOpen => Some(Token::TriangleClose),
            _ => None,
        }
    }
}

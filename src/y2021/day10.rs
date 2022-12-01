use crate::Solution;

pub struct Day10;

#[derive(PartialEq)]
enum Bracket {
    /// ( or )
    Round,
    /// [ or ]
    Square,
    /// { or }
    Curly,
    /// < or >
    Angle,
}

enum Token {
    Open(Bracket),
    Close(Bracket),
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '(' => Token::Open(Bracket::Round),
            '[' => Token::Open(Bracket::Square),
            '{' => Token::Open(Bracket::Curly),
            '<' => Token::Open(Bracket::Angle),
            ')' => Token::Close(Bracket::Round),
            ']' => Token::Close(Bracket::Square),
            '}' => Token::Close(Bracket::Curly),
            '>' => Token::Close(Bracket::Angle),
            _ => return Err(()),
        })
    }
}

enum ParserResult {
    /// The parsed input is complete
    Complete,

    /// The parsed input needs more tokens
    Incomplete,

    /// The parsed input contained this unexpected closing bracket
    Corrupted(Bracket),
}

struct Parser {
    /// A list of opened brackets
    opened: Vec<Bracket>,
}

impl Parser {
    /// Clear the parser state to begin parsing a new input.
    fn clear(&mut self) {
        self.opened.clear()
    }

    /// Parse a list of tokens and return the state of the parser.
    /// Will reuse the previous parser state if not cleared before.
    fn parse(&mut self, tokens: impl Iterator<Item = Token>) -> ParserResult {
        for token in tokens {
            match token {
                Token::Open(b) => self.opened.push(b),

                Token::Close(closing) => {
                    if let Some(opening) = self.opened.pop() {
                        if opening == closing {
                            // Both matches, and the opened bracket has already been popped out of the parser
                            // Continue with next token
                        } else {
                            // Brackets do not match
                            return ParserResult::Corrupted(closing);
                        }
                    } else {
                        // Closing bracket without any opening one
                        return ParserResult::Corrupted(closing);
                    }
                }
            }
        }

        if self.opened.is_empty() {
            ParserResult::Complete
        } else {
            ParserResult::Incomplete
        }
    }
}

impl Solution for Day10 {
    /// Parse each line containing opening and closing brackets.
    /// If a line is corrupted, add its corrupted bracket score to a counter.
    /// Finally, return this score counter.
    fn q1(&self, data: &str) -> String {
        let lines = Self::parse_data(data);
        let mut parser = Parser {
            opened: Vec::with_capacity(32),
        };

        let mut score = 0u64;
        for tokens in lines {
            parser.clear();
            if let ParserResult::Corrupted(b) = parser.parse(tokens) {
                score += match b {
                    Bracket::Round => 3,
                    Bracket::Square => 57,
                    Bracket::Curly => 1197,
                    Bracket::Angle => 25137,
                }
            }
        }

        score.to_string()
    }

    /// Parse as in q1 but keep only the incomplete lines.
    /// For each bracket that needs to be added (in order): `line_score = 5*line_score + bracket_score`
    /// Finally, find the median line_score (odd number of lines).
    fn q2(&self, data: &str) -> String {
        let lines = Self::parse_data(data);
        let mut parser = Parser {
            opened: Vec::with_capacity(32),
        };

        let mut scores = Vec::with_capacity(64);
        for tokens in lines {
            parser.clear();
            if let ParserResult::Incomplete = parser.parse(tokens) {
                let mut line_score = 0u64;
                for b in parser.opened.iter().rev() {
                    line_score *= 5;
                    line_score += match b {
                        Bracket::Round => 1,
                        Bracket::Square => 2,
                        Bracket::Curly => 3,
                        Bracket::Angle => 4,
                    }
                }
                scores.push(line_score)
            }
        }

        // Instead of sorting (O(nlog(n))), we can use the select method to
        // find the median element in O(n)
        let scores_len = scores.len();
        let (_, middle_score, _) = scores.select_nth_unstable(scores_len / 2);
        middle_score.to_string()
    }
}

impl Day10 {
    /// Parse the lines into multiple sets of tokens
    fn parse_data(data: &str) -> impl Iterator<Item = impl Iterator<Item = Token> + '_> + '_ {
        data.split_terminator('\n')
            .map(|line| line.chars().map(|c| c.try_into().unwrap()))
    }
}

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, terminated}
};
use std::collections::HashMap;
use crate::core::instruction::Instruction;

/// Represents a token in the assembly language
#[derive(Debug, PartialEq)]
pub enum Token {
    Label(String),
    Instruction(String),
    Register(String),
    Number(i64),
    String(String),
    Identifier(String),
}

/// Represents a single line of assembly code
#[derive(Debug)]
pub struct AsmLine {
    pub label: Option<String>,
    pub instruction: String,
    pub operands: Vec<Token>,
}

/// Assembler for converting assembly code to VM instructions
#[derive(Default, Debug)]
pub struct Assembler {
    labels: HashMap<String, usize>,
    instructions: Vec<Instruction>,
}

impl Assembler {
    /// Create a new assembler instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Assemble the given source code into VM instructions
    pub fn assemble(&mut self, source: &str) -> Result<Vec<Instruction>, String> {
        self.labels.clear();
        self.instructions.clear();

        // First pass: collect labels and count actual instructions
        let mut instr_count = 0;
        for line_str in source.lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        {
            if let Ok((_, asm_line)) = parse_line(line_str.trim()) {
                if let Some(label) = asm_line.label {
                    self.labels.insert(label, instr_count);
                }
                // Only count if it's not just a label
                if !asm_line.instruction.is_empty() {
                    instr_count += 1;
                }
            }
        }

        // Second pass: generate instructions
        for line_str in source.lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        {
            if let Ok((_, asm_line)) = parse_line(line_str.trim()) {
                if !asm_line.instruction.is_empty() {
                    self.process_instruction(asm_line)?;
                }
            }
        }

        Ok(self.instructions.clone())
    }

    fn process_instruction(&mut self, line: AsmLine) -> Result<(), String> {
        match line.instruction.as_str() {
            // Stack Operations
            "PUSH" => {
                if let Some(Token::Number(n)) = line.operands.get(0) {
                    self.instructions.push(Instruction::Push(*n));
                    Ok(())
                } else {
                    Err("PUSH requires a number operand".to_string())
                }
            }
            "POP" => {
                self.instructions.push(Instruction::Pop);
                Ok(())
            }
            "DUP" => {
                self.instructions.push(Instruction::Dup);
                Ok(())
            }
            "SWAP" => {
                self.instructions.push(Instruction::Swap);
                Ok(())
            }

            // Arithmetic Operations
            "ADD" => {
                self.instructions.push(Instruction::Add);
                Ok(())
            }
            "SUB" => {
                self.instructions.push(Instruction::Sub);
                Ok(())
            }
            "MUL" => {
                self.instructions.push(Instruction::Mul);
                Ok(())
            }
            "DIV" => {
                self.instructions.push(Instruction::Div);
                Ok(())
            }
            "LT" => {
                self.instructions.push(Instruction::LessThan);
                Ok(())
            },
            "LE" => {
                self.instructions.push(Instruction::LessEqual);
                Ok(())
            },
            "GT" => {
                self.instructions.push(Instruction::GreaterThan);
                Ok(())
            },
            "GE" => {
                self.instructions.push(Instruction::GreaterEqual);
                Ok(())
            },

            // Memory Operations
            "LOAD" => {
                if let Some(Token::Identifier(name)) = line.operands.get(0) {
                    self.instructions.push(Instruction::Load(name.clone()));
                    Ok(())
                } else {
                    Err("LOAD requires an identifier operand".to_string())
                }
            }
            "STORE" => {
                if let Some(Token::Identifier(name)) = line.operands.get(0) {
                    self.instructions.push(Instruction::Store(name.clone()));
                    Ok(())
                } else {
                    Err("STORE requires an identifier operand".to_string())
                }
            }

            // Array Operations
            "NEWARRAY" => {
                self.instructions.push(Instruction::NewArray);
                Ok(())
            }
            "ARRAYGET" => {
                self.instructions.push(Instruction::ArrayGet);
                Ok(())
            }
            "ARRAYSET" => {
                self.instructions.push(Instruction::ArraySet);
                Ok(())
            }
            "ARRAYLEN" => {
                self.instructions.push(Instruction::ArrayLength);
                Ok(())
            }
            "FREEARR" => {
                self.instructions.push(Instruction::FreeArray);
                Ok(())
            }

            // String Operations
            "NEWSTR" => {
                if let Some(Token::String(s)) = line.operands.get(0) {
                    self.instructions.push(Instruction::NewString(s.clone()));
                    Ok(())
                } else {
                    Err("NEWSTR requires a string operand".to_string())
                }
            }
            "STRCAT" => {
                self.instructions.push(Instruction::StringConcat);
                Ok(())
            }
            "STRLEN" => {
                self.instructions.push(Instruction::StringLength);
                Ok(())
            }
            "FREESTR" => {
                self.instructions.push(Instruction::FreeString);
                Ok(())
            }

            // Control Flow
            "JMP" => {
                if let Some(Token::Identifier(label)) = line.operands.get(0) {
                    if let Some(&address) = self.labels.get(label) {
                        self.instructions.push(Instruction::Jump(address));
                        Ok(())
                    } else {
                        Err(format!("Label not found: {}", label))
                    }
                } else {
                    Err("JMP requires a label operand".to_string())
                }
            }
            "JMPZ" => {
                if let Some(Token::Identifier(label)) = line.operands.get(0) {
                    if let Some(&address) = self.labels.get(label) {
                        self.instructions.push(Instruction::JumpIfZero(address));
                        Ok(())
                    } else {
                        Err(format!("Label not found: {}", label))
                    }
                } else {
                    Err("JMPZ requires a label operand".to_string())
                }
            }
            "JMPNZ" => {
                if let Some(Token::Identifier(label)) = line.operands.get(0) {
                    if let Some(&address) = self.labels.get(label) {
                        self.instructions.push(Instruction::JumpIfNotZero(address));
                        Ok(())
                    } else {
                        Err(format!("Label not found: {}", label))
                    }
                } else {
                    Err("JMPNZ requires a label operand".to_string())
                }
            }

            // I/O Operations
            "PRINT" => {
                self.instructions.push(Instruction::Print);
                Ok(())
            }
            "PRINTCHAR" => {
                self.instructions.push(Instruction::PrintChar);
                Ok(())
            }
            "PRINTSTR" => {
                if let Some(Token::String(s)) = line.operands.get(0) {
                    self.instructions.push(Instruction::PrintStr(s.clone()));
                    Ok(())
                } else {
                    Err("PRINTSTR requires a string operand".to_string())
                }
            }
            "HALT" => {
                self.instructions.push(Instruction::Halt);
                Ok(())
            }

            _ => Err(format!("Unknown instruction: {}", line.instruction))
        }
    }
}

// Parser functions
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

pub fn number(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(
            pair(
                opt(char('-')),
                digit1
            )
        ),
        str::parse::<i64>
    )(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(
            take_while1(|c| c != '"'),
            String::from
        ),
        char('"')
    )(input)
}

pub fn label(input: &str) -> IResult<&str, Token> {
    map(
        terminated(identifier, char(':')),
        |s: &str| Token::Label(s.to_string())
    )(input)
}

pub fn instruction(input: &str) -> IResult<&str, Token> {
    map(
        identifier,
        |s: &str| Token::Instruction(s.to_uppercase())
    )(input)
}

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((
        map(number, Token::Number),
        map(string_literal, Token::String),
        map(identifier, |s: &str| Token::Identifier(s.to_string())),
    ))(input)
}

pub fn parse_line(input: &str) -> IResult<&str, AsmLine> {
    let (input, _) = multispace0(input)?;
    let (input, label) = opt(terminated(label, multispace0))(input)?;
    let (input, instr) = terminated(instruction, multispace0)(input)?;
    let (input, operands) = many0(
        terminated(
            operand,
            delimited(multispace0, opt(char(',')), multispace0)
        )
    )(input)?;

    Ok((input, AsmLine {
        label: label.map(|token| match token {
            Token::Label(s) => s,
            _ => unreachable!(),
        }),
        instruction: match instr {
            Token::Instruction(s) => s,
            _ => unreachable!(),
        },
        operands,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_assembly() {
        let source = r#"
            start: PUSH 10
            PUSH 20
            ADD
            STORE result
            JMP start
        "#;

        let mut assembler = Assembler::new();
        let instructions = assembler.assemble(source).unwrap();

        assert!(!instructions.is_empty());
        assert_eq!(assembler.labels.get("start"), Some(&0));
    }

    #[test]
    fn test_array_operations() {
        let source = r#"
            // Create and initialize array
            PUSH 5       // array size
            NEWARRAY
            STORE arr

            // Set array[0] = 42
            LOAD arr
            PUSH 0
            PUSH 42
            ARRAYSET

            // Get array[0]
            LOAD arr
            PUSH 0
            ARRAYGET
            PRINT
        "#;

        let mut assembler = Assembler::new();
        let result = assembler.assemble(source);
        assert!(result.is_ok());
    }
}
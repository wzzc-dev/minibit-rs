#[derive(Debug, Clone)]
enum Regex {
    Char(char),
    Concat(Box<Regex>, Box<Regex>),
    Union(Box<Regex>, Box<Regex>),
    // Opt(Box<Regex>),
    Star(Box<Regex>),
    // Plus(Box<Regex>)
}

impl Regex {
    fn compile(&self, labels: &mut Vec<usize>) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match self {
            Regex::Char(c) => {
                instructions.push(Instruction::MatchChar(*c));
                labels.push(1);
            },
            Regex::Concat(exprs1, exprs2) => {
                instructions.extend(exprs1.compile(labels));
                instructions.extend(exprs2.compile(labels));
            },
            Regex::Union(exprs1, exprs2) => {
                labels.push(1);
                let l1 = labels.len();
                let exprs1_compile: Vec<Instruction> = exprs1.compile(labels);
                labels.push(1);
                let l2 = labels.len();
                let exprs2_compile = exprs2.compile(labels);
                let l3 = labels.len();
                labels.push(1);

                instructions.push(Instruction::Split(l1, l2));
                instructions.extend(exprs1_compile);
                instructions.push(Instruction::Jump(l3));
                instructions.extend(exprs2_compile);
                instructions.push(Instruction::End);
            },
            // Regex::Opt(expr) => {
            //     instructions.push(Instruction::Split(instructions.len(), instructions.len() + expr.compile().len()));
            //     instructions.extend(expr.compile());
            //     instructions.push(Instruction::End);
            // },
            Regex::Star(expr) => {
                let l1 = labels.len();
                labels.push(1);
                let l2 = labels.len();
                let expr_compile = expr.compile(labels);
                labels.push(expr_compile.len() + 2);
                let l3 = labels.len();
                labels.push(1);
                instructions.push(Instruction::Split(l2, l3));
                instructions.extend(expr_compile);
                instructions.push(Instruction::Jump(l1));
                instructions.push(Instruction::End);
            },
            
            // Regex::Plus(expr) => {
            //     instructions.push(Instruction::Split(instructions.len(), instructions.len() + expr.compile().len()));
            //     let first = expr.compile();
            //     let loop_label = instructions.len();
            //     instructions.extend(first);
            //     instructions.push(Instruction::Jump(loop_label));
            //     instructions.push(Instruction::End);
            // }
        }
        instructions
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    MatchChar(char),
    Jump(usize),
    Split(usize, usize),
    End,
}

#[derive(Debug, Clone)]
struct State {
    pc: usize, // Program counter
    input:  Vec<char>,
    offset: usize, // Current position in the input string
}

#[derive(Debug)]
struct VM {
    program: Vec<Instruction>,
    state: State,
}

impl VM {
    fn new(program: Vec<Instruction>, input:  Vec<char>) -> Self {
        VM {
            program,
            state: State {
                pc: 0,
                input,
                offset: 0,
            },
        }
    }

    fn run(&mut self) -> bool {
        let mut stack = vec![self.state.clone()];
        while let Some(state) = stack.pop() {
            match self.program[state.pc] {
                Instruction::MatchChar(c) => {
                    if state.offset < state.input.len() && state.input[state.offset] == c {
                        stack.push(State {
                            pc: state.pc + 1,
                            input: state.input,
                            offset: state.offset + 1,
                        });
                    }
                },
                Instruction::Jump(pc) => {
                    stack.push(State {
                        pc,
                        input: state.input,
                        offset: state.offset,
                    });
                },
                Instruction::Split(pc1, pc2) => {
                    stack.push(State {
                        pc: pc2,
                        input: state.input.clone(),
                        offset: state.offset,
                    });
                    stack.push(State {
                        pc: pc1,
                        input: state.input.clone(),
                        offset: state.offset,
                    });
                },
                Instruction::End => {
                    if state.offset == state.input.len() {
                        return true;
                    } else {
                        return false;
                    }
                    
                }
            }
        }
        false
    }
}

fn main() {
    // a|bc*
    let regex = Regex::Union(
        Box::new(Regex::Char('a')),
        Box::new(Regex::Concat(
                    Box::new(Regex::Char('b')), 
                    Box::new(Regex::Star(Box::new(Regex::Char('c'))))
                )
            )
        ); 


    // // a(b|c)
    // let regex = Regex::Concat(
    //     Box::new(Regex::Char('a')),
    //     Box::new(Regex::Union(
    //         Box::new(Regex::Char('b')),
    //         Box::new(Regex::Char('c'))
    //     ))
    // );

    let program = regex.compile(&mut vec![]);
    println!("{:?}", program);
    let mut vm = VM::new(program, "bcd".chars().collect());
    let result = vm.run();
    println!("Match result: {}", result);
}


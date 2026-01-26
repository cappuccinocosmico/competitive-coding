use std::ops::Deref;

use competitive_coding::inputs::ADVENT_INPUT_DAY_6;

enum ArithmeticOperation {
    Sum,
    Prod,
}
struct HomeworkProblem {
    numbers: Vec<i64>,
    operation: ArithmeticOperation,
}
fn try_collect<T, E>(iterator: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut return_vec = Vec::with_capacity(iterator.size_hint().0);
    for item in iterator {
        match item {
            Ok(val) => return_vec.push(val),
            Err(err) => return Err(err),
        }
    }
    Ok(return_vec)
}

impl HomeworkProblem {
    fn execute(&self) -> i64 {
        let lambda_op = match self.operation {
            ArithmeticOperation::Sum => |a: i64, b: &i64| a + b,
            ArithmeticOperation::Prod => |a: i64, b: &i64| a * b,
        };
        let initializer = match self.operation {
            ArithmeticOperation::Sum => 0,
            ArithmeticOperation::Prod => 1,
        };

        self.numbers.iter().fold(initializer, lambda_op)
    }

    fn parse(inputs: &[impl Deref<Target = str>]) -> Result<Self, &'static str> {
        let operation_str = &**inputs.last().ok_or("inputs was empty")?;
        let operation = match operation_str {
            "*" => ArithmeticOperation::Prod,
            "+" => ArithmeticOperation::Sum,
            _ => return Err("last item did not match a valid arithmetic operation"),
        };
        let numbers_iter = inputs[..inputs.len() - 1].iter().map(|v| v.parse::<i64>());
        let return_numbers = try_collect(numbers_iter);
        return_numbers
            .map(|numbers| HomeworkProblem { operation, numbers })
            .map_err(|_| "Integer could not be parsed")
    }
}

fn check_if_row_is_well_formed<T>(
    input: impl Iterator<Item = Option<T>>,
) -> Result<Option<Vec<T>>, &'static str> {
    let mut return_vec = Vec::new();
    for (index, item_opt) in input.into_iter().enumerate() {
        if let Some(item) = item_opt {
            if index != return_vec.len() {
                return Err("Found an empty item in a populated vec");
            };
            return_vec.push(item);
        } else if !return_vec.is_empty() {
            return Err("Found item in an otherwise empty vec.");
        }
    }
    if return_vec.is_empty() {
        Ok(None)
    } else {
        Ok(Some(return_vec))
    }
}

fn parse_problem_1_form(raw: &str) -> Vec<Vec<&str>> {
    fn construct_line_iter<'a>(line_str: &'a str) -> std::str::SplitWhitespace<'a> {
        line_str.split_whitespace()
    }
    let mut parsed_out_newlines: Vec<_> = raw.lines().map(construct_line_iter).collect();
    let mut instruction_lists = Vec::new();
    loop {
        let raw_option =
            check_if_row_is_well_formed(parsed_out_newlines.iter_mut().map(|iter| iter.next()));
        let final_option = raw_option.unwrap();
        let Some(vector) = final_option else { break };
        instruction_lists.push(vector);
    }
    instruction_lists
}

fn parse_problem_2_form(raw: &str) -> Vec<Vec<String>> {
    let mut lines_vec: Vec<_> = raw.lines().collect();
    let mut operations_row = lines_vec.pop().unwrap().split_whitespace();
    let mut iters_mut: Vec<_> = lines_vec.into_iter().map(|v| v.chars()).collect();
    let mut outputs = Vec::new();
    let mut working_output = Vec::new();
    let mut empty_rows_seen_previously = 0;
    while empty_rows_seen_previously <= 10 {
        let row_string: String = iters_mut
            .iter_mut()
            .filter_map(|val| val.next())
            .filter(|char| !char.is_whitespace())
            .collect();
        // dbg!(&row_string);
        if row_string.is_empty() {
            if empty_rows_seen_previously == 0 {
                working_output.push(operations_row.next().unwrap_or_default().to_string());
                let append_vec = std::mem::take(&mut working_output);
                outputs.push(append_vec);
            }
            empty_rows_seen_previously += 1;
        } else {
            empty_rows_seen_previously = 0;
            working_output.push(row_string);
        }
    }
    outputs
}

fn execute_task_1(input: &str) -> i64 {
    let parsed_out_lines = parse_problem_1_form(input);
    let parsed_problems_iter = parsed_out_lines
        .into_iter()
        .map(|instructs| HomeworkProblem::parse(&instructs));
    let problems = try_collect(parsed_problems_iter).unwrap();
    problems.iter().map(|prob| prob.execute()).sum()
}

fn execute_task_2(input: &str) -> i64 {
    let parsed_out_lines = parse_problem_2_form(input);
    let parsed_problems_iter = parsed_out_lines
        .into_iter()
        .map(|instructs| HomeworkProblem::parse(&instructs));
    let problems = try_collect(parsed_problems_iter).unwrap();
    problems.iter().map(|prob| prob.execute()).sum()
}
pub const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

fn main() {
    assert_eq!(execute_task_1(EXAMPLE_INPUT), 4277556);
    println!("Output 1: {}", execute_task_1(ADVENT_INPUT_DAY_6));
    assert_eq!(execute_task_2(EXAMPLE_INPUT), 3263827);
    println!("Output 2: {}", execute_task_2(ADVENT_INPUT_DAY_6));
}

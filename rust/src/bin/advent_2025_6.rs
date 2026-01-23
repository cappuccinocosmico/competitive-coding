struct InstructionLists<'a>(Vec<Vec<&'a str>>);

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

fn parse_from_main_string(raw: &str) -> Vec<Vec<&str>> {
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

fn main() {}

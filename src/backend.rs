use std::collections::HashMap;

pub mod prompt;

pub fn execute(command: String, mut input: String, mut variables: HashMap<String, String>) -> (String, HashMap<String, String>) {
  (input, variables) = prompt::execute(command, input, variables);
  return (input, variables);
}
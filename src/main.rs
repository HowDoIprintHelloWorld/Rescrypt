use std::collections::HashMap;

mod backend;


fn main() {
  let mut vars = HashMap::new();
  let command = "put \"you now\"    %1 why  \"so you know!\"".to_string();
  vars.insert("%1".to_string(), "Lil cease".to_string());
  let (input, variables) = backend::execute(command, "nothing lol".to_string(), vars);
}


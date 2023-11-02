use std::collections::HashMap;

mod backend;



fn main() {
  let mut vars = HashMap::new();
  vars.insert("%1".to_string(), "Lil cease".to_string());
  backend::execute("put \"you now\" %1 \"so you know!\"".to_string(), "nothing lol".to_string(), vars);
}
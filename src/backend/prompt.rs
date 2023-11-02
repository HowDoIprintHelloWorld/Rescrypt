use std::collections::HashMap;

#[path = "./logs.rs"] mod logs;



pub fn execute(command: String, input: String, variables: HashMap<String, String>) -> (String, HashMap<String, String>) {
  println!("{}", command);
  let args = getArgs(command);
  if args.len() == 0 {
    logs::log("No args");
    return (input, variables);
  }
  let func = args[0].clone();
  args = replaceVars(args, variables);
  println!("{:?}", args);
  println!("{}",func);
  println!("{:?}",variables);
  return (input, variables);
}


fn replaceVars(args: Vec<String>, variables: HashMap<String, String>) -> Vec<String> {
  let mut newArgs: Vec<String> = Vec::new();
  for mut arg in args {
    if arg.starts_with('%') {
      if variables.contains_key(&arg) {
        arg = variables.get(&arg).clone().unwrap().to_string();
      }
    } else if arg.starts_with("\\%") {
      arg = arg[1..].to_string();
    }
  }
  return args
}


fn getArgs(command: String) -> Vec<String> {
  let mut args = Vec::<String>::new();
  if command.trim().len() == 0 {
    return args;
  }
  let mut currentArg = String::new();
  let mut isString = false;
  for char in command.chars() {
    if char == '"' {
      isString = !isString
    } else if char == ' ' && !isString {
      if currentArg.is_empty() {
        continue
      }
      args.push(currentArg);
      currentArg = "".to_string();
    } else {
      currentArg.push(char);
    }
  }
  args.push(currentArg);
  return args
}

  
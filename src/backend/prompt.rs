use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, IntoPyDict};

#[path = "./logs.rs"] mod logs;



pub fn execute(command: String, input: String, variables: HashMap<String, String>) -> (String, HashMap<String, String>) {
  println!("{}", command);
  let mut args = getArgs(command);
  if args.len() == 0 {
    logs::log("No args");
    return (input, variables);
  }
  let func = args[0].clone();
  args = replaceVars(args, &variables);
  println!("{:?}", args);
  println!("{}",func);
  println!("{:?}", variables);
  return (input, variables);
}


fn replaceVars(args: Vec<String>, variables: &HashMap<String, String>) -> Vec<String> {
  let mut newArgs: Vec<String> = Vec::new();
  for mut arg in args {
    if arg.starts_with('%') {
      if variables.contains_key(&arg) {
        arg = variables.get(&arg).clone().unwrap().to_string();
      }
    } else if arg.starts_with("\\%") {
      arg = arg[1..].to_string();
    }
    newArgs.push(arg)
  }
  return newArgs;
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


fn runPy()  {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                "def example(*args, **kwargs):
                    if args != ():
                        print('called with args', args)
                    if kwargs != {}:
                        print('called with kwargs', kwargs)
                    if args == () and kwargs == {}:
                        print('called with no arguments')",
                "",
                "",
            ).unwrap()
            .getattr("example")
            .unwrap().into();

            fun.call(py, (), Some([(1, 3)].into_py_dict(py)) );
        // call object without any arguments
        fun.call0(py);
        
        });
    })
}

# Rescrypt concept
It is supposed to be small scripting language/program. It can be used to do small operations to generate or format text and save/write that to a file. It is not necessarily supposed to act like vim or nano, as those are specialised in writing. Rescrypt is meant to be used for formatting and generation of patterns etc.

## Interface and usage
The UI is split into three parts. At the bottom of the screen is the *prompt*, which lets the user type in their commands. These commands can then generate an output based on the users input. The output is shown in the *output window*. Previous outputs can be cycled through and saved in variables. Then, the output can be written to the *file preview*. For example, a numbered list can be created, which can then be inserted into the main program at a specified location. The user can cycle through all of these windows by pressing *option* and the arrow keys. The prompt is at the bottom, the output window to the left and the file preview to the right. 

## Commands
The commands can take variables, modify them and produce an output. The output is written to the output window, which can then be written to the file preview. It can also create things like numbered lists, replace all occurrences of a word or phrase, jump to these or increment all numbers in the text by one.


## Design philosophy
The backend and the UI *must* be separated. The backend must work independently of the UI to allow for several UIs. This ensures interoperability between the different UIs.

## Function execution
You can call a function "put" by writing "put". To pass an argument '1' to the function, you write "put 1". You can also pass entire strings as arguments by writing "put 1 "Hello, friend!"". When enter is hit, the command is executed.  

## Output window
The commands output is written to the output window. It is typically overwritten. The command effects all highlighted text or everything if nothing was highlighted. The effected parts are then replaced with the end result. There can be multiple output windows open at once, like tabs. Text can be loaded into it by either directly typing into it, copying from the file preview or generating using the prompt. 

## Todo
- Prompt
  - accept input (done)
  - turn input into tokens (done)
  - execute functions
  - take arguments
  - replace variables

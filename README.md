# RLisp
Simple Lisp dialect written in Rust currently under development.

This project is a redo of my Math-Lisp repository. The code in that repositiory became messy and hacked together over time, so I decided it would be best to just start from scratch. This interpreter will, ideally, have more structured and readable code.

## How to Build/Run
Install [Rust](https://www.rust-lang.org/) and then navigate to the src directory in command prompt (or terminal). Then run "cargo build" and the project should be built for you. To then run it either type "cargo run" into command prompt or navigate to the created application file.

## How to Use
Because this is a Lisp Interpreter, it makes use of a REPL. Enter your command and hit enter to see the result. Afterwards, repeat.

## Native Functions
Function | Description | Example input | Corresponding output
--- | --- | --- | ---
+,-,*,/ | Basic math functions | (* (+ 19 (- 5 4)) (/ 2 4)) | 10
>,>=,<,<=,= | comparisons | (> 10 4) | T
list | Creates a list containing the passed arguments | (list 1 2 3 4 5 6) | (1 2 3 4 5 6)
car | Returns first element of list | (car (list 1 2 3 4)) | 1
cdr | Returns all but the first element of a list | (cdr (list 1 2 3 4)) | (2 3 4)
cons | cons's the two arguments together | (cons 0 (list 1 2)) | (0 1 2)
nth | returns the nth element of a list | (nth 2 (list 0 1 2 3 4)) | 2
nthcdr | returns all but the first n elements of a list | (nthcdr 3 (list 1 2 3 4 5)) | (4 5)
define | sets value of a symbol | (define x 10) | 10
if | executes statement if condition is not nil | (if (= 5 5) 2 3) | 2

## TODO (in no particular order)
- [ ] Add more native functions
- [ ] Add special forms
- [ ] Add macros
- [ ] Allow user to modify environment
- [ ] Add quote and backquote
- [ ] Make error messages more helpful

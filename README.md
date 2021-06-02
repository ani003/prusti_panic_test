# prusti_panic_test
Repo of Rosetta codes which cause Prusti to panic

## panic_files
`panic_files` directory contains all the [Rosetta codes](https://www.rosettacode.org/wiki/Rosetta_Code) which panicked when running [`prusti-rustc`](https://github.com/viperproject/prusti-dev) on them. Along with the code files, the backtrace produced by executing `prusti-rustc` with `RUST_BACKTRACE=1` is also present in the corresponding `backtrace` files.

This folder was produced using the bash script `run.sh`.

## run.sh
`run.sh` is used to check which Rust programs panic. It takes one argument which is the directory `dir` to be processed. If nothing is passed, `dir` defaults to `.`. The structure of `dir` depends on which flag is passed. 

### Flags
* `-f` or `--files` (Default):
  - `dir` must contain all the `.rs` files which are to be processed.
  - **Output** : If no file panics, nothing is output. If a file panics, new directory  `panic_files` is created in the same directory as `run.sh`. For every `fname.rs` which panics, a subdirectory `fname` is created inside `panic_files`. Inside `fname` three files are present: code file `fname.rs`, the backtrace produced in `backtrace` and a template `README.md`.

* `-d` or `--dirs`:
  - `dir` must contain the subdirectories which are to be processed. Each subdirectory `fname` must contain a `fname.rs` within it.
  - This flag is mainly to reevaluate the `panic_files` after bug fixes.
  - **Output** : If a file `fname.rs` panics, then its backtrace produced is written to its corresponding `backtrace` file present in `fname` subdirectory. If a file `fname.rs` does not panic, then a new directory `no_panic` is created (if it does not exist) in the same directory as `run.sh` and the entire subdirectory `fname` is moved to `no_panic`.
  
### Execution
To execute `run.sh`, first update the second line of the script with the correct absolute location of the `prusti-rustc` binary. Then as per your requirements, run one of the two commands by replacing `dir` with the input directory.
```
$ ./run.sh -f dir
$ ./run.sh -d dir
```

## panic_sources.sh
`panic_sources.sh` is run on the output (`panic_files`) of `run.sh`. It identifies all the unique sources of panic and the programs which have panicked at that place. It outputs a markdown file `panic_sources.md` of this information with a checkbox against every panic source. To run it, type the following command.
```
$ ./panic_sources.sh panic_files
```

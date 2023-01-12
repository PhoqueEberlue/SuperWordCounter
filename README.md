# SuperWordCounter
School project implementing a word counter based on the map-reduce principle.

## Project structure

```
├── benchmark              Folder for benchmark outputs
│   ├── dhat               Valgrind dynamic heap analysis out
│   ├── flamegraph         Flamegraph of the program
│   └── time-comparator    Jupyter Notebook for time comparison
└── src                    Source code of the project
    ├── main.rs            Main file
    └── swc
        ├── mod.rs         Main Module
        ├── mapper.rs      Mapper and modulus functions
        ├── reducer.rs     Reducer function
        └── splitter.rs    Read files and split functions
```

## Building the project

You can either build the project using the `dev` profile, providing prints for debugging, or use `release` for better performances. 

```bash
cargo build --profile=dev
```


## Running the program

```bash
cargo run --profile=dev -- <program arguments>
```

Or simply with the executable

```bash
./target/dev/super_word_counter <program arguments>
```

### Program arguments

```
-i, --input-folder <path>            a folder containing files
-m, --mapper <int>                   the number of mappers
-r, --reducer <int>                  the number of reducers
-l, --to-lower <true|false>          if the words have to be lowered
-f, --modulus-function <len|ascii>   the modulus function that distributes words among reducers
```


### Examples

```bash
cargo run --profile=dev -- -i ./input/folder
cargo run --profile=release -- -i ./input/folder -m 16 -r 16
cargo run --profile=release -- -i ./input/folder -m 7 -r 3 -f len -l false
```


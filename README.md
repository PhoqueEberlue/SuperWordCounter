# SuperWordCounter
School project implementing a word counter based on the map-reduce principle using ~ 🦀 Rust 🦀 ~.

## Table of contents

- [Project structure](Project-structure)
- [Building the project](Building-the-project)
- [Running the program](Running-the-program)
- [How the program works](How-the-program-works)


## Project structure

```
├── benchmark              Folder for benchmark outputs
│   ├── dhat               Valgrind dynamic heap analysis out
│   ├── flamegraph         Flamegraph of the program
│   └── time-comparator    Time comparison with single threaded
└── src                    Source code of the project
    ├── main.rs            Main file
    └── swc
        ├── mod.rs         Main Module
        ├── mapper.rs      Mapper and modulus functions
        ├── reducer.rs     Reducer function
        └── splitter.rs    Read files and split functions
```

## Building the project

To run and build the project you should have the [rust compiler](https://www.rust-lang.org/tools/install) installed.

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

## How the program works 

The program is divided by 3 main phases:
- The splitting part, where we split every file into equals chunks of text
- The mapper, that consists of counting words of a chunks
- The reduce, that gets the outputs of the mappers and merges them together

Among those phases, only the splitting part is performed on a single threaded, the others are concurrent with a
customisable number of threads.

Each instances of threads (excluding main) are referred as either `mappers` or `reducers`.

### Splitting

In order to distribute workload evenly among the different mappers, we first need to get the total size of the folder.
Then we can divide the total number of bytes by the number of mappers chosen by the user. This way we can know exactly
how many bytes each mapper should receive. 

The bytes received by mappers are vectors of unsigned int 8 bits that we called `chunk` previously. Each chunk is then 
filled with the files from the folder until it is full. 

Finally, the main thread launches the mappers, passing them a single chunk for each. 

Note: here, passing variables in the thread is done using the Rust `move` keyword, the thread receive the ownership of 
the variable meaning that it is no longer accessible from the main thread.

### Mapping

The mapper task is to count occurrences of words in the chunks that is provided to it. Following the map-reduce 
principle, we use hashmaps to store those data. A mapper will create as many hashmap that there are reducers. Each 
hashmap representing the data that will be provided to a single reducer.

To chose which word goes into which hashmap, we use what we called `modulus function`. These are functions that takes
as a parameter the word and the number of hashmaps, and returns the hashmap number that the word should go. This task
is achieved by calculating the modulus of the word - len or ascii code - by the total number of hashmaps.

Each mapper then returns the vector containing all hashmaps to the main thread.

The main thread waits for all mappers to finish their tasks before continuing.

### Reducing

Traditionally in the map-reduce algorithm, there is a shuffle task in between the mapping and reducing part. Here it is
fairly simple :

Let n the n-th reducer, give every hashmap at the n index of every vector returned by mappers.

This way, every reducer receive a hashmap containing words following the same criteria given by the modulus function.

After receiving every hashmaps, simply merge them together inside a single hashmap. This hashmap is returned by the 
thread to the main thread.

The main thread waits for every thread to finish and then proceed to merge every hashmap together.
This time the process is way faster, as every hashmap contains different words.

## For the future

It could be interesting to implement a version of this project using sockets in order to distribute mappers and reducers
over a network of computers. The project have been designed to separate those respective functions into different files,
and the file mod.rs only calls 3 functions corresponding to the 3 phases described before. With this architecture, it
is very easy to change the call of each function to a request on another computer and to wait the request(s) before 
starting another phase.

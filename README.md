# Elerons - ELectRONics Search with Rust 
Elerons is a command line tool for electronic component selection.  

## Background

Elerons is designed to be a fast, high perforant search for components on Octopart via the command line. 

For example, on Linux

```Terminal
./elerons "330pF 0603 16V" 
```
While on Windows

```Terminal
elerons.exe "100pF 0402 10V" 
```

There are several Octopart interfaces available, many of them in Python or Javascript. Elerons takes a different approach and employs Rust. The graphql-client crate is used along with reqwest to perform the tasks at hand. The motivation for this approach is the speed, along with the ability to scale the program in the future with async requests. 

Searches are often involved and tedious. Elerons not only does the search, but will apply filters that are based on engineering know how. The usage from the command line allows for individual searches which is often quite useful when developing hardware, but can easily be extended to search an entire BOM for parts. 

## Using 

```Terminal
cargo build 
cd /target/debug
./elerons --help

```
An example of running a search from the command line is shown below, showing the immediate response of the part request query. 

<img src="media/Elerons.gif">


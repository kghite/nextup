<div align="center">

# Nextup

<b>keep your barrier to starting development sessions under control by tracking the next action to take on a 
maximum of 3 active projects</b>

[📑 Usage](#usage) &nbsp; [🛠️ Install](#installation) &nbsp; [📜 License](#license)
</div>

---

## Usage

For best results:

* **check** nextup when starting activity on a project to lower your barrier to entry and avoid losing context when 
  switching between development sessions.

* **update** a nextup when switching away from a project by recording one achievable action to complete 
when you next pick the project up or by simply capturing the state you are leaving things in.

Note there is a fixed maximum of 3 projects to focus efforts.

### Flow

These are some commands you might want to try working into your routine:

`nextup`: lists all current projects and their nextups

```
> nextup

a: complete nextup cli tool
   nextup: update the usage section in README
  
b: my_cool_game development
   nextup: fix the chicken ai - broken pathplanning on level 4

c: FKT the Everest Summit
   nextup: get new legs
```

`nextup set <a, b, c> <title>`: sets a project's title or short description and resets any previous nextup

```
> nextup set a 'build a lunar lander'

set
  a: build a lunar lander
    nextup: ____
```

`nextup <a, b, c> <nextup>`: sets a project's nextup

```
> nextup a 'source an altimeter'

set
  a: build a lunar lander
     nextup: source an altimeter
```

`nextup <a, b, c>`: reports the nextup for a project

```
> nextup a

a: build a lunar lander
  nextup: source an altimeter
```

`nextup reset`: resets all projects and nextups

```
> nextup reset

a: ____
   nextup: ____
  
b: ____
   nextup: ____

c: ____
   nextup: ____
```


### Analysis

**TODO** - Usage analysis across project lifespan


## Installation

### From Binary Release

You can find the binary files in the releases for this repository.

**On Ubuntu:** Install with 

    sudo install ./Downloads/nextup /usr/bin

### From [crates.io](crates.io)

    cargo install nextup

### From Source

You can `cargo run` this repository, adding any arguments with `--`.  To enable the logger, set `RUST_LOG=debug`.

**Example:**

    env RUST_LOG=debug cargo run --bin nextup -- a 'what's nextup for project a'

### Where is my data going?

Project data is stored in the standard user-invisible configuration file location for your operating 
system.  For more information and path lookups see the [`directories` crate](https://crates.
io/crates/directories). 

Log data powering the analysis features is stored in a local SQLLite database file as described in the docs for the 
[turbosql crate](https://docs.rs/turbosql/latest/turbosql/#wheres-my-data).

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

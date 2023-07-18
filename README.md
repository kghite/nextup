<div align="center">

# Nextup

---

<b>keep your barrier to starting development sessions under control by tracking the next action to take on a 
maximum of 3 active project</b>

[🛠️ Install](#installation) &nbsp; [📑 Usage](#usage) &nbsp; [📜 License](#license)
</div>

---

## Installation

**From Binary Release**

**From Source**

---

## Usage

For best results:

* **check** nextup when starting activity on a project to lower your barrier to entry and avoid losing context when 
  switching between development sessions.

* **update** a nextup when switching away from a project by recording one achievable action to complete 
when you next pick the project up or by simply capturing the state you are leaving things in.

Note there is a fixed maximum of 3 projects to focus efforts.

**Flow**

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
    nextup: <not set>
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

a: <not set>
   nextup: <not set>
  
b: <not set>
   nextup: <not set>

c: <not set>
   nextup: <not set>
```


**Analysis**

---

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

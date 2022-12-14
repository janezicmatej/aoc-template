![ci](https://github.com/janezicmatej/aoc-template/actions/workflows/ci.yml/badge.svg)
# Advent-of-Code {YEAR}
*This is a dumbed down version of [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) with some extra features*

## Project overview

### Project structure
- `src/` :
    - `bin/`:
        - `<day>.rs`: solution files 
        - `*.rs`: convenience scripts
    - `inputs/`: this directory is gitignored, input files go here
    - `examples/`: example files go here; you can push this as test are run in ci
    - `utils/`: utils files go here
    - `lib.rs`: contains framework code
    - `main.rs`: contains framework code
- `.env.example`: example dotenv file

### Cli
- `cargo scaffold <day>`: prepare solution files for `day`
- `cargo download <day>`: download input file for `day`
- `cargo solve <day>`: run solution against input for `day`
- `cargo all`: alias for run; runs solutions for all days


### dotenv

set `YEAR` to whichever year you are solving for and `TOKEN` to AoC session Cookie

### FAQ

#### How are your commits numbered in ascending order?
[https://westling.dev/b/extremely-linear-git](https://westling.dev/b/extremely-linear-git)

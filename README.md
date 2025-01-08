# recursive-cpu-usage

Simple utility to get the CPU used by a process and all of its children / grand-children / ...

Done by measuring the time the process was scheduled in user and kernel mode over a 100 ms interval.
Note that the output will be inaccurate if the child processes are very short lived.

## Installation

`cargo install --git https://github.com/FliegendeWurst/recursive-cpu-usage`

## Usage

Execute: `recursive-cpu-usage <PID>`

```
> recursive-cpu-usage 1234
 0.5
> recursive-cpu-usage 81925
10.5
```
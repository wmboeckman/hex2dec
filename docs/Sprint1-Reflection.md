# Sprint 1 Reflection
#### Feb. 14th 2026

**Story Points Completed**: Base-conversion algorithms, basic CLI functionality

## Summary

The first sprint went over quite smoothly, with my Story Point estimations being perhaps too high. I was able to complete the core features of this sprint fairly early on, and started to work ahead.

This caused a bit of a mess at the end of the sprint, and there are a few known bugs with these lately-added features (see below). What I should have accounted for was a separate "input processing / sanitization" story to account for the headache I've been having about handling the edge cases in user input.

That all being said, we now have an easy to use cli tool that lets you convert between and base 8, 10, or 16 number!

![cli-screenshot](img\2026-02-13.png)

### Known Bugs

- "0x" returns 0. should return new custom error!
- very large numbers seem to fail in conversion (probably `usize` is the culprit)
- index out-of-bounds error:
    - "0" results in panic at util.rs:24:63. this should be interpreted as base-10
    - file io: reading lines containing just a '0' panics

## Looking Forward

As I begin to integrate these systems together, I will have to decide how each component will interact with each other. Currently, I have split the program into a few different modules (CLI, file io, custom errors, conversion algorithms, tests, general utility functions). That said, I believe some of this code can be consolidated further into a set of 2-3 modules (General IO, Conversion, Utils).

Ideally, I want each module to avoid calling out to other modules directly, and implement custom error definitions where applicable. This will no doubt require some refactoring, and therefore adjusted the upcoming sprints in accordance.

### Potential Additions

- file io: debug should print line number alongside error
- negative integers fail as "invalid character" due to negative sign. update to ignore for now?
- rewrite outputs + error handler to use the [log](https://docs.rs/log/latest/log/index.html) library.
    - *This seems very easy to add, might just drop this in before the start of next sprint!*
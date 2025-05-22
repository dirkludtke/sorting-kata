# Sorting Kata
Algorithmic kata for building and testing algorithms — using sorting as example

## Background

Sometimes, the simplest solution to a problem isn’t the most efficient — either in terms of speed
or memory use. On the other hand, more optimized algorithms can become so complex that it’s hard
to understand how they work, making them especially tricky to debug.

This creates a bit of a chicken-and-egg situation. Simple algorithms can be tested with data we
create by hand, but more advanced implementations often require more complex test cases — ones that
are too tedious or difficult to create manually.

One effective strategy is to start with a simple, correct solution. Even if it's slow, it can be
used to generate test data for the more sophisticated versions. In this kata, we take that idea
even further, ... and do it four times!

## Side goals

1. get a feeling for computational complexity (Big O notation)
1. get to know several sorting algorithms with their advantages and disadvantages

## How to

The kata has four stages. Each stage stands for a runtime complexity: O(n**2),
O(n log n), O(n log n) worst case, and O(n).

- The [testdata](testdata) folder contains test data for the stages. Input and output data
  are in separate files. The output data is empty at the beginning. You start by creating
  the output data of stage 1 by hand.
- Each of the programming languages have separate files for each of the stages (stage1,
  stage2, stage3, stage4). Please start by implementing stage 1.
- There is a programs to test your implementation (sortTest/sort_test) on the available
  test data. Run it without arguments to get usage info.
- You can generate the test data of the next stage with (sortExecute/sort_execute). These
  programs write to stdout, you have to redirect the output to the correct output file.
- Now you can implement the next stage.

Sorting of small lists and arrays is fast, even with bad algorithms. For seeing the tiny
runtime differences, we punish each comparison with a virtual 1-second delay. If sorting
needs more than one virtual hour (which corresponds to 3600 comparisons), the test or,
respectively, execution fails.

The test data is created in a way, that an algorithm of stage n can handle the data of all
stages until n + 1. You can use it to create the test data of the next stage but you need
to implement the next stage before you can move on.

## Usage Details

### Java
We tested with Java 21.0.7. For generating and running .class files next to their .java
source files:
- cd sorting-kata/java
- javac SortExecute.java (or, respectively, SortTest.java)
- java SortExecute (or respecitively, SortTest)

### Python
We tested with Python 3.12.3. Just run the scripts:
- python sorting-kata/python/sort_execute.py (or, respectively, sort_test.py)

### Rust
We tested with Rust 1.87.0. It works. But as we are not very confident in Rust, please help to improve:
- cd sorting-kata/rust
- cargo run --bin sort_execute (or, respectively, sort_test)

### Typescript
We tested with Node 24.0.1 using its native Typescript support. As this is still an
experimental features, Node generates a warning at the beginning. You can suppress it
with the --no-warnings flag. Please note that Node versions prior to 22.6.0 do not
support this at all, and that you need the flag --experimental-strip-types before
version 23.6.0.
- node --no-warnings sorting-kata/typescript/sortExecute.ts (or, respectively, sortTest.ts)

## Material

Wikipedia has some really great articles for depening on the topics of this kata.

sorting in general
: https://en.wikipedia.org/wiki/Sorting_algorithm

bubble sort
: https://en.wikipedia.org/wiki/Bubble_sort

insertion sort
: https://en.wikipedia.org/wiki/Insertion_sort

quicksort
: https://en.wikipedia.org/wiki/Quicksort

complexity theory
: https://en.wikipedia.org/wiki/Computational_complexity_theory

big O notation
: https://en.wikipedia.org/wiki/Big_O_notation


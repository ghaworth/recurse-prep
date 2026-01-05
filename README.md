# Recurse Center Preparation

Preparation work for my Recurse Center application, focusing on the pairing interview tasks.

## Goal

Complete one or more of the pairing interview tasks in Rust, demonstrating both the conceptual understanding and the ability to extend the solution under pressure.

## Current Progress

### Lisp Parser

Write code that takes Lisp code and returns an abstract syntax tree.

**Status:** Python implementation complete. Rust implementation in progress.

- `lisp-parser/python/lisp_parser.py` — working Python version
- `lisp-parser/rust/` — Rust port (in progress)

**Example:**

Input: `"(first (list 1 (+ 2 3) 9))"`

Output: `["first", ["list", 1, ["+", 2, 3], 9]]`

## Other Pairing Tasks (Not Yet Started)

- Tic Tac Toe
- Space Invaders
- Database Server
- Symbolic Differentiator

## Why Rust?

Embedded systems is the target. Walking into the interview with Rust signals intentionality about that direction.

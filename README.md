# Garrote

> An esoteric lang designed to garrote your brain

Garrote is a **queue-based** esolang. It uses... well, queues for its entire memory model

# Table of Contents

- [Introduction](#introduction)
- [Hello, World!](#hello-world)
- [Memory Model](#memory-model)
- [Execution Model](#execution-model)
- [Instruction Set](#instruction-set)
- [Examples](#examples)
- [License](./LICENSE.md)

# Introduction

Garrote is a queue-based esoteric programming language (esolang) that uses a single FIFO (First-In, First-Out) queue as its memory model. Every operation is performed by manipulating values within this queue.

Unlike most esolangs that torture you with stacks, Garrote tortures you with a queue.

# Hello, World!

_Coming soon_

# Memory Model

Garrote uses a single FIFO (First-In, First-Out) queue as its memory model. Values are always pushed to the back and removed from the front.

Each value in the queue is an unsigned 8-bit integer — it can hold `0` to `255`. If a value exceeds `255` it wraps around, so `256` becomes `0`, `257` becomes `1`, and so on.

If you need to hold a value, it's somewhere in the queue. good luck getting to it.

# Execution Model

Garrote executes instructions left-to-right, one at a time.

Control flow is handled entirely through **bookmarks** — a separate stack that records positions in the program. `%` marks a position, `^` jumps back to it (or doesn't, depending on the front of the queue). Loops nest correctly because the bookmark stack is LIFO: the innermost `%` is always on top and resolves first.

```
program:   3  %  &  1  -  ^  #
              ^             |
              |             |
              +-------------+
                jumps back here if front != 0
```

Because `^` peeks instead of pops, `%...^` behaves as a **do-while** — the body always runs at least once before the first real check.

Nested loops work the same way — the bookmark stack just gets deeper:

```
bookmark stack

  [ inner % ]  <-- resolves first
  [ outer % ]  <-- resolves after inner exits
```

# Instruction Set

|     Token     | Name     | Effect                                                                                                                                                                                 |
| :-----------: | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `n` (literal) | push     | Push the literal `n` onto the back of the queue.                                                                                                                                       |
|      `#`      | pop      | Remove and discard the front of the queue. No replacement.                                                                                                                             |
|      `~`      | graft    | Clones the first value in the queue and enqueues it.                                                                                                                                   |
|      `+`      | fuse     | Pop the front two values, push their sum, and reverses the queue.                                                                                                                      |
|      `-`      | split    | Pop the front two values, push `max(a - b, 0)`, and reverses the queue — clamped at zero, never negative.                                                                              |
|      `%`      | bookmark | Push the current position onto a separate bookmark stack. Does not touch the queue.                                                                                                    |
|      `^`      | jump     | Peek (don't pop) the front of the queue. If nonzero → jump back to the position on top of the bookmark stack. If zero → pop the bookmark stack, then continue to the next instruction. |
|      `&`      | print    | Prints the first value as an ASCII character.                                                                                                                                          |

## Details

**`+` and `-` pop order** — both pop the front value first (`a`), then the new front (`b`). For `+` it doesn't matter. For `-` it does: result is `max(a - b, 0)`, not the other way around.

```
queue: [ 3 | 10 | ... ]
         a    b

-  →  max(3 - 10, 0)  →  0
```

**`+` wraps on overflow** — values are u8, so `250 + 10` gives `4`, not `260`.

**`-` clamps at zero** — if `a < b`, you get `0`.

**`%` doesn't touch the queue** — if your queue looks wrong near a loop, `%` is not the culprit.

**`^` is a peek, not a pop** — the value driving the loop stays on the front of the queue the whole time. if you want it gone after the loop exits, pop it manually with `#`.

# Examples

_Coming soon_

# License

See [LICENSE](./LICENSE).

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

```
72&#101&#108&&#111&#44&#32&#87&#111&#114&#108&#100&#33&
```

You might wonder...

> What the f\*ck am I looking at?

Calm down, let's read this instruction-by-instruction

```
72&#   load 'H', print, pop
101&#  load 'E', print, pop
108&&# load 'L', print twice, pop
111&#  load 'O', print, pop
44&#   load ',', print, pop
32&#   load ' ', print, pop
87&#   load 'W', print, pop
111&#  load 'O', print, pop
114&#  load 'R', print, pop
108&#  load 'L', print, pop
100&#  load 'D', print, pop
33&    load '!', print
```

it loads an unsigned 8-bit integer and displays it as an ASCII character

# Comments

Anything that is not an instruction/a literal is ignored. Making them act like a comment

```
72&# <this is a valid comment>
71 <this is a valid comment too!> &#
```

# Memory Model

Garrote uses a single FIFO (First-In, First-Out) queue as its memory model. Values are always pushed to the back and removed from the front.

Each value in the queue is an unsigned 8-bit integer — it can hold `0` to `255`. If a value exceeds `255` it wraps around, so `256` becomes `0`, `257` becomes `1`, and so on.

If you need to hold a value, it's somewhere in the queue. good luck getting to it.

# Execution Model

Garrote executes instructions left-to-right, one at a time.

Control flow is handled entirely through **bookmarks** — a separate stack that records positions in the program. `%` marks a position, `^` jumps back to it (or doesn't, depending on the front of the queue). Loops nest correctly because the bookmark stack is LIFO: the innermost `%` is always on top and resolves first.

```
program:   3  %  &  1  -  ^  #
              ^           |
              |           |
              +-----------+
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

**`+` and `-` pop order** — both pop the front value first (`a`), then the new front (`b`). For `+` it doesn't matter. For `-` it does: result is `a.wrapping_sub(b)`, not the other way around.

```
queue: [ 3 | 10 | ... ]
         a    b

-  →  max(3 - 10, 0)  →  0
```

**`+` and `-` wraps on overflow** — values are u8, so `250 + 10` gives `4`, not `260`.

**`%` doesn't touch the queue** — if your queue looks wrong near a loop, `%` is not the culprit.

**`^` is a peek, not a pop** — the value driving the loop stays on the front of the queue the whole time. if you want it gone after the loop exits, pop it manually with `#`.

# Examples

_Coming soon_

# License

See [LICENSE](./LICENSE).

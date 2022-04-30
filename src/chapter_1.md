# Introduction

> What are we building?

We will build a multithreaded **R**ead **E**val **P**rint **L**oop.

`TODO: image of three threads - main, user input, worker - with arrows between them`

> Why?

The goal is learning how to build an app that can:
* do multiple tasks concurrently
  - consume user input without blocking
  - parse commands that may have errors without crashing
* share knowledge between tasks
* is beginner friendly
  - no async
  - no borrows
  - no locks
  - no mutexes

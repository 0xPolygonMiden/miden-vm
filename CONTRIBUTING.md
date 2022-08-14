# Contributing to Miden

#### First off, thanks for taking the time to contribute!

You can find a detailed explanation of main project concepts in [docs](https://maticnetwork.github.io/miden/).

We want to make contributing to this project as easy and transparent as possible, whether it's:

- Reporting a [bug](https://github.com/maticnetwork/miden/issues/new)
- Take part in [discussions](https://github.com/maticnetwork/miden/discussions)
- Submitting a fix
- Proposing new [features](https://github.com/maticnetwork/miden/issues/new)

&nbsp;

## Flow
We are using [Github Flow](https://guides.github.com/introduction/flow/index.html), so all code changes happen through pull requests.

### Versioning
We use [semver](https://semver.org/) naming convention.

### Branching
- The current active branch is `next`. Every branch with fix/feature must be forked from `next`.

- Branch name should contain a short issue/feature decription separated with hyphens [(kebab-case)](https://en.wikipedia.org/wiki/Letter_case#Kebab_case).

For example, if the issue title is `Fix functionality X in component Y` then branch name will be smth like: `fix-x-in-y`.

- New branch should be rebased from next before submitting a PR in case there have been changes to avoid merge commits.
i.e. this branches state:
```
          A---B---C fix-x-in-y
         /
    D---E---F---G next
            |   |
         (F, G) changes happened after `fix-x-in-y` forked
```
should become this after rebase:
```
                  A'--B'--C' fix-x-in-y
                 /
    D---E---F---G next
```
More about rebase [here](https://git-scm.com/docs/git-rebase) and [here](https://www.atlassian.com/git/tutorials/rewriting-history/git-rebase#:~:text=What%20is%20git%20rebase%3F,of%20a%20feature%20branching%20workflow.)


- Also would be good to squash commits to logically separated, distinguishable stages to keep git log clean.
i.e.
```
7hgf8978g9... Added A to X \
                            \  (squash)
gh354354gh... oops, typo --- * ---------> 9fh1f51gh7... feat(X): Add A && B
                            /
85493g2458... Added B to X /


789fdfffdf... Fixed D in Y \
                            \  (squash)
787g8fgf78... blah  blah --- * ---------> 4070df6f00... fix(Y): fixed D && C
                            /
9080gf6567... Fixed C in Y /
```

### Commit messages
Commit messages should be written in short and descriptive manere, prefixed with changes type and scope(if possible) tags according to [semantic commit](https://gist.github.com/joshbuchea/6f47e86d2510bce28f8e7f42ae84c716) scheme.

### Code Style and Documentation
- For documentation in the codebase, we follow the [rustdoc](https://doc.rust-lang.org/rust-by-example/meta/doc.html) convention with no more than 100 characters per line.
- For code sections, we use code separators of 100 characters like the following:
```
// CODE SECTION HEADER
// ================================================================================
```
[Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) linting included in CI pipeline, but, it will not be superfluous to run them locally.

&nbsp;

## Pre-PR checklist
1. Repo forked and branch created from `next` according to the naming convention.
2. Commit messages and code style follow conventions.
3. Tests added for new functionality.
4. Documentation/comments updated according to changes.
5. Clippy and Rustfmt linting passed.
6. New branch rebased from `next`.

&nbsp;

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

&nbsp;

## Any contributions you make will be under the MIT Software License
In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.
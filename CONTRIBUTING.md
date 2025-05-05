# Contributing

## Tools and dependencies

The easiest way to get started is to install [Mise](https://mise.jdx.dev).
The project uses mise-en-place to manage tool dependencies and environment variables. It is also our task runner.

The [installation instructions](https://mise.jdx.dev/docs/installation) are available on the Mise website.

## Bootstrap

After installing Mise, you can bootstrap the project by running `mise bootstrap`.

### Build tasks

The full list of commands can be displayed at any time by running `mise tasks ls`.

#### Common tasks

- `mise bootstrap`: Bootstrap the project
- `mise lint`: Run all linting tasks
- `mise fmt`: Run all formatting tasks
- `mise fix`: Fix all linting problems that can be automatically fixed
- `mise test`: Run all tests
- `mise build`: Build the project

#### Running specific tools

If you need to run a specific tool, some commands are available in the `mise fmt:<tool>` or `mise fmt:<tool>:<extra>` format.

For example `mise fmt:rustfmt` will run `rustfmt` on the project.

## Git-hooks

The project uses [Lefthook](https://github.com/evilmartian/lefthook) for git hooks.

### Installation

Mise will install Lefthook for you, you just need to run `mise install`.
But you can also install it manually by following the instructions in [Lefthook's documentation](https://lefthook.dev/installation/).
Note that Lefthook will try to install mise-en-place for you.

#### Usage

After installation, you can run `lefthook install` to install the hooks.

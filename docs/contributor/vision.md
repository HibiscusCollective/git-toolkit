# Project Vision

Git Toolkit extends Git's user experience to be more friendly and approachable while integrating tightly with the conventional
commits specification.
The project aims to reduce friction in collaborative Git workflows by streamlining repetitive tasks like adding co-authors and
ticket references to commit messages.
By providing a set of lightweight, configurable tools that fit seamlessly into existing Git workflows, Git Toolkit subtly guides
users toward best practices without imposing rigid processes.

## Core Objectives

| Priority | Objective                                             | Success Metric                                          |
|----------|-------------------------------------------------------|---------------------------------------------------------|
| P0       | Simplify adding ticket references to commit messages  | Reduced time spent on commit message preparation        |
| P0       | Maintain compatibility with existing Git workflows    | Zero conflicts with standard Git commands and processes |
| P1       | Enable easy addition of co-authors to commit messages | Increased usage of co-author attribution in commits     |
| P1       | Provide configurable templates for commit messages    | User-reported satisfaction with customization options   |
| P2       | Save and retry failed commit messages                 | Reduced frustration when commit hooks fail              |
| P3       | Interactive commit builder based on message templates | User adoption of the interactive builder                |

## Governance

The Git Toolkit project follows
the [Hibiscus Collective governance model](https://github.com/HibiscusCollective/.github/wiki/Governance). Major decisions
follow a collaborative, inclusive, and transparent process as outlined in
the [Decision Making Process](https://github.com/HibiscusCollective/.github/wiki/POL%E2%80%900001:-Decision-Making-Process).

The project is committed to:

- Open and transparent development
- Inclusive community participation
- Respectful and constructive discourse
- Zero tolerance for hate speech, discrimination, and intolerance
- Security and stability as core values

As the project grows, we aim to transition to a more community-driven governance model, with clear paths for contributors to
take on leadership roles.

## Out of scope

The following list of features is explicitly out of scope of the project.
Community forks are absolutely welcome and encouraged to provide them, but it would take a very compelling argument to add them
to the project.

Note that this is not an exhaustive list. Items may be added or removed as the project evolves.

### Git workflow enforcement

Git Toolkit aims to guide users toward best practices but explicitly avoids enforcing specific Git workflows or processes. The
tools should remain helpful suggestions rather than mandatory gates in the development process.

### Replacement for existing Git tools

Git Toolkit complements Git's existing functionality rather than replacing it. We do not aim to create alternative
implementations of core Git commands or functionality.

### Complex UI or IDE integration

While we may provide simple terminal-based interfaces, creating complex UIs or deep IDE integrations is out of scope. The focus
is on command-line tools that integrate with Git's existing interface.

# Tech Strategy

Git Toolkit follows
the [Hibiscus Collective tech strategy](https://github.com/HibiscusCollective/.github/wiki/ADR%E2%80%900001:-Primary-Programming-Language)
with Rust as the primary programming language. This document outlines the specific technical approach for Git Toolkit.

## Technical Pillars

### Simplicity

- Command-line tools should be intuitive and follow Git's existing patterns
- Configuration should use familiar formats and locations
- Features should solve specific pain points without adding complexity
- Code should be straightforward, well-documented, and easy to maintain

### Performance

- Tools must be lightweight and fast-loading
- Operations should complete with minimal latency
- Resource usage should be minimal, especially for frequently used commands
- Avoid unnecessary dependencies that could impact startup time or memory usage

### Adaptability

- Support tiered configuration (global, project-level, command-line overrides)
- Allow customization of templates and message formats
- Enable integration with different project management systems
- Respect existing Git configurations and workflows

## Architecture

Git Toolkit is structured as a Rust workspace with multiple crates:

1. **Core Libraries**: Shared functionality for Git operations, configuration management, and template handling.
  These can be consumed as crates by other projects too.
2. **Command-line Tools**: Individual binaries for specific features (ticket references, co-authors, etc.)
  These may be combined into a single cli tool in the future, but we will keep offering them as separate tools as well.
  This will allow users to choose the features they want to use and compose them into their desired workflow.
3. **Configuration**: Handlers for the tiered configuration system.
  The config will be shared for all the tools so it makes sense to have it in a distinct crate.

### Design Principles

- **Modular Design**: Each feature should be implemented as a separate tool that can be used independently
- **Composition**: Tools should be composable to create more complex workflows
- **Extension**: Design for extensibility to allow for future features and community contributions
- **Testing**: Comprehensive test coverage for core functionality and edge cases

## Technology Choices

### Primary Language: Rust

As outlined in [ADR-0001](https://github.com/HibiscusCollective/.github/wiki/ADR%E2%80%900001:-Primary-Programming-Language),
Rust provides:

- Memory safety without garbage collection
- High performance with minimal runtime overhead
- Strong type system to prevent bugs at compile time
- Excellent concurrency support via Tokio
- Cross-platform compatibility

### Key Dependencies

- **Tokio**: For async runtime capabilities (if required)
- **Clap**: For command-line argument parsing
- **Config**: For configuration management
- **Git2-rs**: For Git operations that require libgit2 bindings
- **Serde**: For serialization/deserialization of configuration

## Implementation Strategy

1. **Incremental Development**: Start with the ticket reference feature, then add co-author and saved message functionality
2. **Test-Driven Development**: Write tests before implementing features
3. **Documentation as a first class citizen**: Document APIs and user interfaces as they are being built
4. **Early User Feedback**: Release early versions to gather feedback from real users

## Integration Points

- Git hooks, ex: pre-commit, prepare-commit-msg. (To be confirmed)
- Git configuration files
- Git message templates
- Project management systems (via configurable APIs)

## Future Technical Considerations

- Interactive commit builder with TUI
- Conventional commit validation
- Changelog generation based on conventional commits
- Potential for plugin architecture to allow community extensions
- Integration with project management systems with suitable APIs to aid with completion (ex: Jira search, GitHub issues and
 teams, etc)

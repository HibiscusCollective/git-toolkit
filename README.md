# Git Toolkit

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/HibiscusCollective/git-toolkit/badge)](https://scorecard.dev/viewer/?uri=github.com/HibiscusCollective/git-toolkit)

Git Toolkit extends Git's user experience to be more friendly and approachable while integrating tightly with the conventional commits specification. It aims to reduce friction in collaborative Git workflows by streamlining repetitive tasks like adding ticket references to commit messages.

**Note:** This project is a work in progress. Currently, only the ticket reference functionality is supported.

## Development

Git Toolkit uses [mise](https://github.com/jdx/mise) to manage development tools and tasks. To get started:

1. Install mise if you haven't already
2. Bootstrap the project:

   ```bash
   mise run bootstrap
   ```

### Common Development Tasks

- **Build the project**: `mise run build`
- **Run tests**: `mise run test`
- **Format code**: `mise run fmt`
- **Lint code**: `mise run lint`
- **Fix linting issues**: `mise run fix`

For cross-platform builds, you can use:

- `mise run build:debug:linux` - Build for Linux
- `mise run build:debug:windows` - Build for Windows
- `mise run build:release` - Build release versions

## License

Copyright (C) 2025 Pierre Fouilloux, Hibiscus Collective

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  
See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  
If not, see <https://www.gnu.org/licenses/>.

[Full text of the license](LICENSE)

## Third party software

This project wouldn't have been possible without the work of many talented and dedicated craftspeople around the world. Thank you.

All the software used in this project is either free software or has a compatible licence.
Check the [.attribution](.attribution) directory for their licences.

Due diligence was taken to ensure the compatibility of the licenses and that our use is permitted, that said mistakes can happen.
If you come across a problem, please [contact us](https://github.com/HibiscusCollective) and let us know. We will work with you to resolve it promptly.

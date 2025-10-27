# DevDash - Developer TUI Dashboard

DevDash is an interactive terminal-based dashboard built with Rust that displays real-time metrics and information for developers. It centralizes productivity data, Git repository status, CI/CD build monitoring, test coverage, and system resource usage in a lightweight and responsive TUI interface.

## Features

### Git Status
- Current branch name and tracking information
- Commits ahead/behind remote branches
- Staged, unstaged, and untracked file counts
- Last commit message and author

### Build Monitor
- Integration with GitHub Actions and GitLab CI
- Real-time build status (success, failure, running)
- Recent build history with timestamps
- Commit SHA and message for each build

### Test Coverage
- Integration with `cargo-llvm-cov` for Rust projects
- Line coverage percentage
- Per-file coverage breakdown
- Visual progress indicators

### System Statistics
- CPU usage percentage
- RAM usage (used/total with percentage)
- Disk space monitoring
- System uptime tracking
- Auto-refresh every 2 seconds

### Pomodoro Focus Timer
- 25-minute default work sessions
- Start/pause/reset functionality
- Task tagging support
- Session history with JSON persistence
- Daily time tracking

### Beautiful Themes
Choose from 6 carefully crafted color schemes:
1. **Moonlight** - Purple/blue gradient (default)
2. **Nord** - Arctic blue tones
3. **Dracula** - Purple/pink dark theme
4. **Gruvbox** - Warm retro colors
5. **Solarized Light** - Cream/blue light theme
6. **Tokyo Night** - Deep blue/cyan night theme

## Installation

### Prerequisites
- Rust 1.70 or later
- `cargo` package manager

### Build from Source

```bash
git clone https://github.com/guicybercode/devdash_rust
cd devdash_rust
cargo build --release
```

The binary will be located at `target/release/devdash`.

## Configuration

1. Copy the example configuration:
```bash
cp config.example.json config.json
```

2. Edit `config.json` with your settings:
```json
{
  "theme": "moonlight",
  "github_token": "ghp_your_github_token",
  "gitlab_token": "glpat_your_gitlab_token",
  "repo_path": ".",
  "github_repo": "owner/repo",
  "gitlab_project_id": "12345",
  "timer_default_minutes": 25
}
```

### API Token Setup

**GitHub:**
- Go to https://github.com/settings/tokens
- Generate a new token with `repo` scope
- Add to `github_token` field

**GitLab:**
- Go to https://gitlab.com/-/profile/personal_access_tokens
- Create a token with `read_api` scope
- Add to `gitlab_token` field

## Usage

Run DevDash:
```bash
cargo run
# or
./target/release/devdash
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Q` | Quit the application |
| `Esc` | Exit gracefully |
| `Tab` | Navigate between panels |
| `Ctrl+T` | Cycle through themes |
| `Space` | Start/pause timer |
| `R` | Refresh all data |
| `?` | Show help (coming soon) |

## Layout

```
+-------------------------------------------------------------+
| DevDash - Painel do Projeto: my-awesome-repo               |
+----------------+----------------+----------------+----------+
| Git Status     | Build Status   | Test Coverage | Timer    |
| - branch: main | - ✅ build ok  | - 87% lines   | [25:00]  |
| - 2 commits ↑  | - ⏳ running… | - 5 arquivos  | [Running]|
+----------------+----------------+----------------+----------+
| Logs / Eventos Recentes                                     |
| - Commit: "fix: ajuste no parser"                           |
| - Build iniciado há 2min                                    |
+-------------------------------------------------------------+
```

## Requirements

- Linux, macOS, or Windows (Windows Terminal recommended)
- Terminal that supports ANSI colors
- Minimum terminal size: 80x24
- Git repository for full feature support

## Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation
- `git2` - Git operations
- `reqwest` - HTTP client for API calls
- `sysinfo` - System information
- `serde/serde_json` - JSON serialization
- `tokio` - Async runtime
- `chrono` - Time handling
- `anyhow` - Error handling

## License

BSD-3-Clause License

Copyright (c) 2025, Guilherme

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

나를 먹으려면, 내가 곧 생명의 양식이니 사람의 인자는 누구든지 나를 먹으면 영생하리라 나를 주려면, 이것은 하늘에서 내려온 양식이니 조상들이 먹고도 죽은 그것과 같지 아니하니라

"For my flesh is meat indeed, and my blood is drink indeed. He that eateth my flesh, and drinketh my blood, dwelleth in me, and I in him. As the living Father hath sent me, and I live by the Father: so he that eateth me, even he shall live by me." (John 6:55-57)

---

made by moonguip gui기กีギ

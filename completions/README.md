# Shell Completions

Shell completions for `lob` command.

## Installation

### Bash

Add to `~/.bashrc` or `~/.bash_profile`:

```bash
source /path/to/lob/completions/lob.bash
```

Or copy to system directory:
```bash
sudo cp lob.bash /usr/share/bash-completion/completions/lob
```

### Zsh

Add to `~/.zshrc`:

```zsh
fpath=(/path/to/lob/completions $fpath)
autoload -Uz compinit && compinit
```

Or copy to system directory:
```bash
sudo cp _lob /usr/local/share/zsh/site-functions/_lob
```

### Fish

Copy to Fish completions directory:

```bash
cp lob.fish ~/.config/fish/completions/
```

## Generation

Completions are generated using `clap_complete`. To regenerate:

```bash
cargo run -- --generate-completions bash > completions/lob.bash
cargo run -- --generate-completions zsh > completions/_lob
cargo run -- --generate-completions fish > completions/lob.fish
```

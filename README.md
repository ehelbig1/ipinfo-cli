## Git Configuration

This repository uses it's own git configuration to define a standard git commit message template and pre-commit hook. The template can be found in the .gitmessage file and all git hooks are defined in the hooks/ directory. To enable this configuration run:

```bash
git config --local include.path ../.gitconfig
```
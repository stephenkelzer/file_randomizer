# file_randomizer
CLI to randomize file names in a directory


### Prerequisites:
- Install [`Rust`](https://www.rust-lang.org/tools/install)
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Available Commands:
_The CLI runs in "dry-run" mode by default. To actually rename the files, use the `--execute` flag._

```
   cargo -- <PATH> [--execute]
```

```
   cargo -- --help
```
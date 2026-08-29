# Rust-Learn — lesson workspace

Workflow:

- Create a new lesson file with the helper script:

```bash
./init.sh topic_name
```

- Edit `src/bin/topic_name.rs` and write your code. Each file is an independent binary.
- Run the lesson:

```bash
cargo run --bin topic_name
```

Notes:

- Put shared helpers in `src/lib.rs` and call them from lesson binaries.
- For tiny snippets you can also use `examples/` and run with `cargo run --example name`.

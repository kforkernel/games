# Games Collection

A collection of mini command-line games written in **C** and **Rust** for practice and fun.

## How to Run

### Prerequisites
* **Rust:** [Install Rust & Cargo](https://rust-lang.org)
* **C:** GCC or Clang compiler

### Cloning Repo
```bash
git clone https://github.com/kforkernel/games
cd games
```

### Running Rust Games
If using a Cargo workspace inside `/rust`:
```bash
cd rust
cargo run -p <game_name>
```

If running standalone game folders:
```bash
cd rust/<game_name>
cargo run
```

### Running C Games
Navigate to a game folder, compile with `gcc`, and run the executable:
```bash
cd c/game_name
gcc main.c -o game
./game
```

---

## License

This project is under the kforkernel Use License (KUL) v1.0.
For more details, read [LICENSE.md](LICENSE.md).

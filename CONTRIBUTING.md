# Contributing

Thank you for helping improve Class Roster.

## Development workflow

1. Create a focused branch from `main`.
2. Install dependencies with `npm ci`.
3. Make the smallest change that solves the issue.
4. Run the frontend and Rust checks before opening a pull request.
5. Explain the behavior changed and include interface screenshots when relevant.

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features
```

## Data safety

- Use fictional names and IDs for development and screenshots.
- Do not commit generated `database.xlsx` files or other personnel records.
- Do not add logging that exposes form values or local file paths.
- Keep application permissions and external dependencies to the minimum required.

## Code style

- Prefer clear names and small functions over explanatory comment blocks.
- Keep the frontend dependency-light and usable in both left-to-right and right-to-left layouts.
- Validate data at the Rust command boundary even when it is already checked in JavaScript.
- Add or update tests whenever backend validation behavior changes.

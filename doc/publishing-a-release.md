# Publishing a Release

1. Clone this repository.
2. Install `pre-commit` hooks.

   ```shell
   pre-commit install
   ```

3. Add feature/patch.
4. Update `README.md` with relevant info.

   💡 Use [`cargo-markdown`][cargo-markdown] to verify that it renders correctly
   on [crates.io][crates-io]

   ```shell
   cargo markdown README.md
   ```

5. Push changes:

   ```shell
   git add .
   git commit -m "feat: shiny new things!"
   git push origin main
   ```

   > This will trigger GitHub Actions to run:
   >
   > 1. `cargo check`
   > 2. `cargo fmt`
   > 3. `cargo clippy`
   > 4. `cargo test`
   > 5. `cargo publish`

   See [`ci.yml`][action-ci].

6. Bump `version` in `Cargo.toml`to `v#.#.#`.

   💡 The pre-commit hooks should take care of running `cargo update`.

7. Push changes:

   ```shell
   git add .
   git commit -m "chore: bump version"
   git push origin main
   ```

8. Tag the last commit:

   ```shell
   git tag v#.#.#
   git push origin v#.#.#
   ```

   > This will trigger GitHub Actions to:
   >
   > 1. Build binaries for Apple Silicon and Intel.
   > 2. Create archives of them using `tar`.
   > 3. Create a draft release in GitHub.
   > 4. Upload the archives to the draft release.

9. Add release notes and publish the draft release.

   💡 The tag and title should be `v#.#.#`.

10. Verify API tokens are still valid.

- Check `HOMEBREW_GITHUB_TOKEN` on [GitHub][tokens-github].
- Check `CARGO_REGISTRY_TOKEN` on [crates.io][tokens-crates-io].

11. Manually run the [`publish`][action-publish] action.

    💡 Make sure to set `Use workflow from` to `v#.#.#`.

    > This will publish `readstor` to:
    >
    > 1. [tnahs/homebrew-forumlas][formulas]
    > 2. [crates.io][crates-io]

[action-ci]: ../.github/workflows/ci.yml
[action-publish]: ../.github/workflows/publish.yml
[cargo-markdown]: https://crates.io/crates/cargo-markdown
[crates-io]: https://crates.io
[formulas]: https://github.com/tnahs/homebrew-forumlas
[tokens-github]: https://github.com/settings/tokens
[tokens-crates-io]: https://crates.io/settings/tokens

# Publishing a Release

1. Clone this repository.
2. Add feature/patch.
3. Bump `version` in `Cargo.toml`to `[VERSION]`.
4. Update `README.md` with relevant info.

   💡 Use [`cargo-markdown`][cargo-markdown] to verify that it renders correctly
   on [crates.io][crates-io]

5. Check for packaging issues with:

   ```console
   cargo publish --dry-run --allow-dirty
   ```

   > Note:
   >
   > - `--dry-run` runs checks without publishing.
   > - `--allow-dirty` ignores any uncommitted changes.

6. Push changes:

   ```console
   git add .
   git commit -m "feat: shiny new things!"
   git push origin main
   ```

   > This will trigger a GitHub Actions to run some CI tests.

7. Tag the last commit:

   ```console
   git tag [VERSION]
   git push origin [VERSION]
   ```

   > This will trigger GitHub Actions to:
   >
   > 1. Build binaries for Apple Silicon and Intel.
   > 2. Create archives of them using `tar`.
   > 3. Create a draft release in GitHub.
   > 4. Upload the archives to the draft release.

8. Add release notes and publish the draft release.

   💡 The tag and title should be `[VERSION]`.

9. Manually run the [`publish`][publish] action.

   💡 Make sure to set `Use workflow from` to `[VERSION]`.

   > This will publish `readstor` to:
   >
   > 1. [tnahs/homebrew-forumlas][formulas]
   > 2. [crates.io][crates-io]

[cargo-markdown]: https://crates.io/crates/cargo-markdown
[crates-io]: https://crates.io
[formulas]: https://github.com/tnahs/homebrew-forumlas
[publish]: https://github.com/tnahs/readstor/actions/workflows/publish.yml

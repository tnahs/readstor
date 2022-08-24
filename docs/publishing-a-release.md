# Publishing a Release

## GitHub

1. Merge feature branch or push any changes to [readstor's](https://github.com/tnahs/readstor) `main`.
2. Run `scripts/run-build-release.sh`. This creates the `release` directory and places three items inside it:
   - `readstor` - The binary.
   - `readstor-mac.tar.gz` - An archive of the binary.
   - `readstor-mac.sha256` - A hash of the archive.
3. Create a [new release](https://github.com/tnahs/readstor/releases/new) in GitHub.
4. Create a new tag with the latest version number e.g. `v0.2.0`.
5. Set the release title to the latest version number e.g. `v0.2.0`.
6. Use this template for the release notes:

   ```markdown
   ## Changes

   ## Features

   ## Bug Fixes
   ```

7. Attach `readstor-mac.tar.gz` and `readstor-mac.sha256`.
8. Publish.

## Homebrew

1. Clone the [homebrew-readstor](https://github.com/tnahs/homebrew-readstor) repository.
2. Open `readstor.rb`.

   ```ruby
   # readstor.rb

   class Readstor < Formula
     desc "A CLI for Apple Books annotations"
     homepage "https://github.com/tnahs/readstor"
     url # URL : string
     sha256 # HASH : string
     license any_of: ["MIT", "Apache-2.0"]
     version # VERSION : string

     def install
       bin.install "readstor"
     end
   end
   ```

3. Replace `URL` with the direct link to the latest release on GitHub. It should look something like:

   ```plaintext
   https://github.com/tnahs/readstor/releases/download/v0.2.0/readstor-mac.tar.gz
   ```

4. Replace `HASH` with the hash from `readstor-mac.sha256`. It should have been printed to the terminal after running the build script.
5. Replace `VERSION` with the latest version number e.g. `v0.2.0`.
6. Commit and push changes to [homebrew-readstor's](https://github.com/tnahs/homebrew-readstor) `main` branch.

## crates.io

1. Run `cargo login`.
2. If asked, input your [crates.io](https://crates.io) API Token or create a new one one [here](https://crates.io/settings/tokens).
3. Run `cargo publish --dry-run` to run all check without uploading.
4. Finally publish with `cargo publish`.

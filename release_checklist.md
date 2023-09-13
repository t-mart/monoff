# Release Checklist

1. Make changes.
2. Bump the version in `Cargo.toml`.
3. Commit.
4. Push.
5. Create a version variable with the new version from `Cargo.toml`:

   ```pwsh
   $monoff_version = "<version>"
   ```

   For example:

   ```pwsh
   $monoff_version = "1.2.3"
   ```

6. Build the program, generate a checksum file off it, and create a release on GitHub:

   ```shell
   cargo build --release
   cd target/release
   sha256sum monoff.exe > monoff.exe-checksum.sha256.txt
   gh release create --generate-notes $($monoff_version) monoff.exe monoff.exe-checksum.sha256.txt
   ```

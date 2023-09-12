# Release Checklist

1. Make changes.
2. Bump the version in `Cargo.toml`.
3. Commit.
4. Push.
5. Build the program, and generate a checksum off it:

   ```shell
   cargo build --release
   cd target/release
   sha256sum monoff.exe > monoff.exe-checksum.sha256.txt
   ```

6. Create a version variable:

   ```pwsh
   $monoff_version = "<version>"
   ```

7. Create a new release with uploaded assets:

   ```shell
   gh release create --generate-notes $($monoff_version) target/release/monoff.exe target/release/monoff exe-checksum.sha256.txt
   ```

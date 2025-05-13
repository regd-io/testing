# Release Checklist for regd-testing

Before tagging a new release (e.g. vx.y.z):

## ✅ Code Quality
- [ ] `cargo fmt -- --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test` passes on Linux/macOS/Windows (check GitHub Actions)
- [ ] `cargo doc` builds without warnings

## ✅ Crate Metadata
- [ ] Update `Cargo.toml`:
  - [ ] version bump
  - [ ] authors, repository, description, categories
- [ ] Check README version and examples are up to date
- [ ] Update any necessary documentation

## ✅ Prepare Release
- [ ] Create a PR with the following commit message: `chore(release): prepare to vx.y.z`
  - [ ] Merge PR once approved

## ✅ Packaging
- [ ] `cargo publish --dry-run` completes successfully
- [ ] Dependencies are accurate and up-to-date
- [ ] `LICENSE` and `README.md` are included in the crate

## 🚀 Release
- [ ] Commit and tag: `git tag vx.y.z && git push origin vx.y.z`
- [ ] `cargo publish`

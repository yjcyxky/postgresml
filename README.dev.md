## Add new functions

pgml-extension/src/kge.rs
pgml-extension/sql/pgml--2.8.2--2.8.3.sql


## Config the environment

### Install cargo-pgrx

```bash
cargo install cargo-pgrx --version "0.11.2" --locked
```

### Install dependencies

```bash
pip install -r requirements.macos.txt 
```

## Test

```bash
# Launch the conda environment
conda activate network-medicine

cd pgml-extension 
cargo test --package pgml --lib -- kge::tests::pg_test_transe_l2 --exact --nocapture 
cargo test --package pgml --lib --features pg_test -- kge::tests --nocapture
```

## Tag and Push to the repository

```bash
git add --all
git commit -m "Add new functions" -a

# Get the last commit hash
git log -1 --pretty=format:%h

# Tag the commit
git tag v2.8.3-<commit-id>

# Push the commit and the tag
git push origin tgmc-master --tags
```



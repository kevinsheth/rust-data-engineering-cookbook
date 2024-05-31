# Rust Data Engineering Cookbook

Some recipes I've developed along the way while learn Rust.

# Set Up

## Authenticate with Google Cloud Platform.

```bash

gcloud auth application-default login
gcloud auth application-default set -quota-project <project-id>

```

## Nix Only

```bash

nix-shell -p pkg-config openssl
```

# Running Recipes

Navigate to a particular parent directory and run.

```bash
cargo run --bin <recipe_name>

```

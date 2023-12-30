This is my first experience in writing smart sontracts.

I used [Solana](https://solana.com/)

### Deps
- rust: `sudo pacman -S rustup`
- [solana](https://docs.solana.com/cli/install-solana-cli-tools) for local development

### Install
    git clone https://github.com/hiebyshek/solana-smart-contract/
    cd solana-smart-contract

### Build
    cargo build-sbf

### Test
run solana-test-validator

[deploy](https://docs.solana.com/cli/deploy-a-program) program

then run:

    cargo test-sbf

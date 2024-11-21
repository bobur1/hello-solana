# Solana Anchor Program Documentation: Hello Solana Program

## Program Overview

This is a simple Solana Anchor program designed to demonstrate basic functionality while incorporating a `security.txt` file for program metadata.

## Deployment Commands

To deploy this program to the Solana blockchain, follow these steps:

1. Build the program:

```bash
anchor build
```

2. Deploy the program:

```bash
anchor deploy
```

3. Check the deployment: (Confirm the program ID matches the declared ID)

```bash
solana program show 8FLVYw42CfMjcbYuvB6tKz2Ms4Pe6QXoaReqZEoY1NkQ
```

## Testing Commands

To test the program functionality, use the following commands:

1. Run tests using Anchor: Ensure the local validator is running before testing:

```bash
anchor test
```

run docker:
`docker buildx build --platform linux/amd64 -t testing:v0 --load .`

`docker run --platform linux/amd64 -d -p 8080:8080 testing:v0`

anchor build --verifiable --solana-version 1.18.23

solana-verify get-executable-hash target/verifiable/hello_solana.so

solana-verify get-executable-hash target/deploy/hello_solana.so

solana-verify get-program-hash DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo

solana program deploy ./target/deploy/hello_solana.so --program-id DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo --keypair ./key/key.json
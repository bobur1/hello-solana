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


# Solana expand program size (for redeploy):

```bash
solana program extend DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo  20000 -u d -k ./key/key.json
```

# Solana Deploy verified

```bash 
solana program deploy ./target/deploy/hello_solana.so --program-id DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo --keypair ./key/key.json
```

solana-verify verify-from-image -e target/deploy/hello_solana.so -i ellipsislabs/hello_solana_verifiable_build:latest -p DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo


# Solana verify

```bash
solana-verify verify-from-repo https://github.com/bobur1/hello-solana --url {RPC_URL} --program-id DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo --mount-path programs/hello-solana --library-name hello_solana --commit-hash c09ae94c09f4e3b49c657c7b816c2981c3e63468
```

```bash
solana-verify verify-from-repo -u "{RPC_URL}" --program-id DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo https://github.com/bobur1/hello-solana --library-name hello_solana --mount-path .
```
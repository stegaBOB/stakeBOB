# stakeBOB

---

## An escrow-less Metaplex NFT staking program built on Solana (Unstable and untested lel. Don't use yet...)

---

### Build the rust program alone
```
$ yarn build:rust
```

---

### Generate the JS SDK and rebuild IDL only (using shank and solita)
```
$ yarn solita
```

---

### Build the JS SDK only (must be generated first)
```
$ yarn build:sdk
```

---

### Build the program and generate/build the IDL/SDK/docs
```
$ yarn build
```

---

### Start Amman and run the test script
Run the following command in a separate shell
```
$ amman start
```

Then, run the Amman script
```
$ yarn amman
```
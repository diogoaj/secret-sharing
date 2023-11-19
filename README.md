# secret-sharing
An implementation of Shamir Secret Sharing scheme in Rust. The scheme is used to share secrets based on polynomial interpolation over finite fields, in this case the field is GF(2^8). 

### Usage:
```
Usage: secret-sharing <COMMAND>

Commands:
  split    
  combine  
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example:
Splitting a secret in 5 shares where only 2 are needed to reconstruct it:
```
$ echo "VerySecret" | ./secret-sharing split -n 5 -t 2
150-43651d45d3f837e66147
144-df6530a51612718ab59f
211-9365e629415b5a509c30
71-f26592bc8269c5e0d44b
103-f36539c2de77c19678a7
```

Combining different shares and recovering the secret:
```
$ head -n 2 shares | ./secret-sharing combine -t 2
Resulting secret: VerySecret
```
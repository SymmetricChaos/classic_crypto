# classic_crypto

Implementations of well known classical ciphers and codes with the ability to accept arbitrary utf-8 characters.



## All implemented ciphers

### Beaufort Family
* beaufort
* progressive key
* autokey
* running key

### Vigenere Family
* vigenere
* progressive key
* autokey
* running key

### Porta Family
* porta
* progressive key
* running key

### Tableaux Family
* tableaux
* progressive key
* autokey
* running key

### Monoalphabetic
* affine
* atbash
* caesar
* decoder ring
* substitution

### Playfair Family
* playfair
* two square
* four square
* slidefair

### Transposition Ciphers
* columnar
* rail fence
* scytale
* route
  * stripe
  * snake

### Other
* alberti cipher disk
* straddling checkerboard
* polybius square
* enigma (M3 variant)
* AFGVX and ADFGX
* bifid
* nihilist


## All implemented codes

### Binary Codes
* ASCII
* bacon
* baudot
* fibonacci
* morse
* UTF32
* Base64

### Spelling Alphabets
* NATO
* CCB



# Coming Soon!
* maybe optimize things? idk enough about Rust, really
* VIC cipher
* turning grille cipher
* more route ciphers
* beaufort variant cipher family
* completion of porta cipher family
* UTF8 and UTF16 encodings
* text to text version of Base64 (but like efficient and stuff)
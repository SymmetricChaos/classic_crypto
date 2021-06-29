# classic_crypto

Implementations of well known classical ciphers and codes with the ability to accept arbitrary utf-8 characters.



## All implemented ciphers

### Monoalphabetic Family
* affine
* atbash
* caesar
* decoder ring
* substitution

### Polyalphabetic Family 
* vigenere
* beaufort
* variant beaufort
* tableaux

#### Polyalphabetic Modifiers
* autokey
* progressive key

### Porta Family
* porta
* progressive key
* running key

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
* grille
* turning grille

### Other
* alberti cipher disk
* straddling checkerboard
* polybius square
* enigma (M3 variant)
* AFGVX and ADFGX
* bifid and trifid
* nihilist
* M209


## All implemented codes

### Binary Codes
* ASCII
* bacon
* baudot
* fibonacci
* morse
* UTF32
* UTF8
* Base64

### Spelling Alphabets
* NATO
* CCB



# Coming Soon!
* maybe optimize things? idk enough about Rust, really
* VIC cipher (in progress)
* RS44 cipher (in progress)
* disrupted columnar transposition
* more route ciphers
* completion of porta cipher family
* UTF16 encoding
* text to text version of Base64 (but like efficient and stuff)
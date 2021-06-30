# classic_crypto

Implementations of well known classical ciphers and codes with the ability to accept arbitrary utf-8 characters.



## All implemented ciphers

### Monoalphabetic Family
* caesar
* affine
* general substitution
* atbash
* decoder ring

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
* seriated playfair

### Transposition Ciphers
* columnar
* rail fence
* scytale
* route
  * stripe
  * snake
* grille
* turning grille

### Composite Ciphers
* enigma (M3 variant)
* AFGVX and ADFGX
* bifid and trifid
* nihilist

### Other
* alberti cipher disk
* straddling checkerboard
* polybius square (generalized)
* chaocipher
* BATCO
* DRYAD
* baziers
* M94
* M209

## All implemented codes

### Binary Codes
* ASCII
* bacon
* baudot
* fibonacci
* morse (ITU)
* UTF8
* UTF32
* Base64

### Spelling Alphabets
* NATO
* CCB

### Romanization
* hirigana and katagana

### Other Codes
* Godel


# Coming Soon!
* maybe optimize things? idk enough about Rust, really
* VIC cipher (in progress)
* RS44 cipher (in progress)
* disrupted columnar transposition
* more route ciphers
* integration of Porta with polyalphabetic
* UTF16 encoding
* text to text version of Base64 (but like efficient and stuff)
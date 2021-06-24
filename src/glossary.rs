/*
Digraph: A pair of characters.
Trigraph: A triple of characters.
n-graph: A tuple of n characters.


Cipher: A method of changing one text to another using a method controlled by a value (or values) known as the key.
    Structs with the trait Cipher must implement the methods .encrypt() and .decrypt()
Code: A method of changing one text to another used a fixed method.
    Structs with the trait Code must implement the methods .encode(), .decode(), and .char_map()

Reciprocal Cipher: A cipher for which the process of encryption is precisely the same as decryption. This is extremely valuable for mechanical ciphers as it simplifies design.
    This family includes: Beaufort, M209, Enigma, Atbash

Transposition Cipher: Ciphers that changes the position of the symbols in the text.
    This family includes: Scytale, Rail Frence, Columnar, Grille, Turning Grille, Route
Substitution Cipher: Ciphers that changes the symbols of the text into different symbols but does not move them around.

Monoalphabetic Cipher: Substitution ciphers where there is a 1-to-1 replacement of symbols in the plaintext and ciphertext. From a cryptanalysis standpoint these ciphers are all effectively identical.
    This family includes: Affine, Atbash, Caesar, DecoderRing, Substitution, the Enigma Plugboard
Polyalphabetic Cipher: Substitution ciphers where the method of replacement is changed repeatedly. Usually this means changing the method with every letter but some ciphers encode the changes into the ciphertext.
    This family includes: Vigenere, Beaufort, Beaufort Variant, Porta, Tableaux, Bazeries, Cipher Disk, Enigma, M209, Chaocipher

Composite Cipher: Ciphers formed by iteratively applying some sequence of ciphers.
Cyclic Key Cipher: Ciphers in which the information from the key is repeated as needed for encryption.
Auto Key Cipher: Ciphers in which the information from the plaintext is incorporated into the key.




Fractionation: Converting individual symbols of plaintext into multiple symbols of ciphertext. This provides no extra security on its own but is powerful when combined with disperson.
    Methods of accomplishing this include: Polybius Square (or higher dimensional version), Straddling Checkerboard, various codes
Dispersion: Information from the plaintext should influence mutiple parts of the ciphertext. This can be accomplished by transposition of a fractionated cipher.


*/
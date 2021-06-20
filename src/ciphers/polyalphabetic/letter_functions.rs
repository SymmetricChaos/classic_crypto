// Vigenere

// The two versions of Beaufort are reciprocal so no dec function is needed
#[inline(always)]
pub fn beaufort_enc(c: usize, k: usize, length: usize) -> usize {
    (length+k-c)%length
}

#[inline(always)]
pub fn beaufort_var_enc(c: usize, k: usize, length: usize) -> usize {
    (length+c-k)%length
}

// Nihilist
#[inline(always)]
pub fn nihilist_enc(c: usize, k: usize) -> usize {
    c+k
}

#[inline(always)]
pub fn nihilist_dec(c: usize, k: usize) -> usize {
    c-k
}
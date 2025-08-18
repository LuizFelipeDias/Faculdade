pub fn busca_linear(vetor: &[i32], alvo: i32) -> Option<usize> {
    for (i, &valor) in vetor.iter().enumerate() {
        if valor == alvo {
            return Some(i);
        }
    }
    None
}
pub fn busca_binaria(vetor: &[i32], alvo: i32) -> Option<usize> {
    let mut inicio = 0;
    let mut fim = vetor.len();

    while inicio < fim {
        let meio = inicio + (fim - inicio) / 2;
        if vetor[meio] == alvo {
            return Some(meio);
        } else if vetor[meio] < alvo {
            inicio = meio + 1;
        } else {
            fim = meio;
        }
    }
    None
}
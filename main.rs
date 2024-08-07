use rand::Rng;

fn main() {
    const TAMANHO_TABULEIRO: usize = 10;

    // Inicializa o tabuleiro com "_"
    let mut tabuleiro: Vec<Vec<char>> = vec![vec!['_'; TAMANHO_TABULEIRO]; TAMANHO_TABULEIRO];

    // Função para posicionar navios aleatoriamente (a ser implementada)
    fn posicionar_navios(tabuleiro: &mut Vec<Vec<char>>) {
        // ... Implementação aqui
    }

    // Função para o jogador atirar (a ser implementada)
    fn atirar(tabuleiro: &mut Vec<Vec<char>>) {
        // ... Implementação aqui
    }

    // Função para imprimir o tabuleiro (a ser implementada)
    fn imprimir_tabuleiro(tabuleiro: &Vec<Vec<char>>) {
        // ... Implementação aqui
    }

    // Posiciona os navios
    posicionar_navios(&mut tabuleiro);

    // Loop principal do jogo
    loop {
        // Imprime o tabuleiro
        imprimir_tabuleiro(&tabuleiro);

        // Permite o jogador atirar
        atirar(&mut tabuleiro);

        // Verifica se o jogo acabou
        // ...
    }
}

fn posicionar_navios(tabuleiro: &mut Vec<Vec<char>>) {
    // ... (código para gerar coordenadas aleatórias e verificar sobreposição)

    // Exemplo de posicionar um navio horizontalmente
    let x = rand::thread_rng().gen_range(0..TAMANHO_TABULEIRO);
    let y = rand::thread_rng().gen_range(0..TAMANHO_TABULEIRO - 1);
    for i in 0..2 {
        tabuleiro[x][y + i] = 'S';
    }
}

use std::fs;
pub fn twenty_two() {
    let caminho = "src/pe_22/names.txt";

    let alfabeto = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let conteudo = fs::read_to_string(caminho).expect("Falha ao ler arquivo");

    let mut nomes: Vec<String> = conteudo.split(",").map(|f| f.replace("\"", "")).collect();
    nomes.sort();

    let mut soma = 0;

    for i in 0..nomes.len() {
        let nome = nomes.iter().nth(i).unwrap();
        let posicao = i + 1;
        let mut pontuacao = 0;
        for c in nome.chars() {
            pontuacao += alfabeto.iter().position(|x| x == &c).unwrap() + 1;
        }
        pontuacao *= posicao;
        soma+=pontuacao;
    }
    println!("{soma}");
}

use super::read::get_contents;

struct Instrucao {
    quantos: i32,
    de_qual: i32,
    pra_onde: i32
}

// 1 to return instructions
// 2 to return drawing
fn separar_instrucoes(path: String, what: i32) -> Vec<String> {
    let arquivo = get_contents(path);
    let mut drawing: Vec<String> = Vec::new();
    let mut instructions: Vec<String> = Vec::new();

    for linha in arquivo.lines() {
        for palavra in linha.split_whitespace() {
            if palavra == "move".to_string() {
                instructions.push(linha.to_string());
                break;
            }
            drawing.push(linha.to_string());
            break;
        }
    }

    match what {
        1 => return instructions,
        2 => return drawing,
    }

}

// Pegar as instrucoes separadas e popular um vetor de struct Instrucao
// com as informações de cada instrucao
fn ler_instrucao() {
}

// Ler um vetor de struct Instrucoes
// e mudar o desenho de acordo com as instrucoes
fn executar_instrucao() {
}

// Ver quais caixas estao no topo de cada pilha
pub fn caixas_no_topo(path: String) -> String {
    let instrucoes: Vec<String> = separar_instrucoes(path, 1);
    "Não terminei".to_string()
}

use super::read::get_contents;

struct Instrucao {
    quantos: i32,
    de_qual: i32,
    pra_onde: i32
}

fn separar_instrucoes(path: String) {
    let arquivo = get_contents(path);
    let mut desenho: String = String::new();
    let mut instrucoes: String = String::new();

    for linha in arquivo.lines() {
        for palavra in linha.split_whitespace() {
            if palavra == "move".to_string() {
                instrucoes.push_str(linha);
                instrucoes.push('\n');
                break;
            }
            desenho.push_str(linha);
            desenho.push('\n');
            break;
        }
    }

    println!("{}", desenho);
    println!("{}", instrucoes);
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
    "Não terminei".to_string()
}

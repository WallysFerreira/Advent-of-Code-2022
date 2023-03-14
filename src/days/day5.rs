use super::read::get_contents;

#[derive(Debug)]
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

    if what == 1 {
        instructions
    } else {
        drawing
    }
}

// Pegar as instrucoes separadas e popular um vetor de struct Instrucao
// com as informações de cada instrucao
fn ler_instrucao(instructions: Vec<String>) {
    let mut numeros: Vec<i32> = Vec::new();

    for i in 0..instructions.len() {
        let palavra: &str = &instructions[i];

        if !palavra.parse::<i32>().is_err() {
            println!("AAAA");
            numeros.push(palavra.parse::<i32>().expect("Erro"));
        }
    }

    println!("{:?}", numeros);

    let mut instrucoes_separadas: Vec<Instrucao> = Vec::new();

    for i in (0..numeros.len()).step_by(3) {
        let j = i + 1;
        let k = j + 1;

        let instruction: Instrucao = Instrucao {
            quantos: numeros[i],
            de_qual: numeros[j],
            pra_onde: numeros[k]
        };

        println!("{:?}", instruction);
    }
}

// Ler um vetor de struct Instrucoes
// e mudar o desenho de acordo com as instrucoes
fn executar_instrucao() {
}

// Ver quais caixas estao no topo de cada pilha
pub fn caixas_no_topo(path: String) -> String {
    let instrucoes: Vec<String> = separar_instrucoes(path, 1);

    ler_instrucao(instrucoes);

    "Não terminei".to_string()
}

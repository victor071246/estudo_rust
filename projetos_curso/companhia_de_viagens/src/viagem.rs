// Crie uma struct para armazenar os dados dos passageiros
pub struct Dados_Passageiros{
    pub nome: String,
    pub numero_passaporte: String,
    pub idade: u8
}

// Crie uma struct para armazenar os dados dos voos
pub struct Dados_Voos{
    pub codigo_voo: String,
    pub partida: String,
    pub destino: String,
    pub data_partida: String,
    pub horario_partida: String
}

// Crie uma função para adicionar um novo passageiro
pub fn adicionar_passageiro(dados_passageiros: &mut Vec<Dados_Passageiros>, nome: String, numero_passaporte: String, idade: u8){
    let passageiro = Dados_Passageiros{nome, numero_passaporte, idade};
    dados_passageiros.push(passageiro);
}

// Crie uma função para adicionar um novo voo
pub fn adicionar_voo(dados_voos: &mut Vec<Dados_Voos>, codigo_voo: String, partida: String, destino: String, data_partida: String, horario_partida: String){
    let voo = Dados_Voos{codigo_voo, partida, destino, data_partida, horario_partida};
    dados_voos.push(voo);
}

// Crie uma função para exibir todos os voos
pub fn exibir_voos(dados_voos: &Vec<Dados_Voos>){
    for voo in dados_voos{
        println!("Código do Vôo: {}", voo.codigo_voo);
        println!("Partida: {}", voo.partida);
        println!("Destino: {}", voo.destino);
        println!("Data de partida: {}", voo.data_partida);
        println!("Horário de partida: {}", voo.horario_partida);
    }
}

// Crie uma função para exibir todos os passageiros
pub fn exibir_passageiros(dados_passageiros:&Vec<Dados_Passageiros>){
    for passageiro in dados_passageiros{
        println!("Nome do passageiro: {}", passageiro.nome);
        println!("Idade do passageiro: {}", passageiro.idade);
        println!("Número de passaporte: {}", passageiro.numero_passaporte);
    }
}
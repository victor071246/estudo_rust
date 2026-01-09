struct Block{
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn new(index: u64, timestamp: u64, data: String, hash: String, prev_hash: String) -> Block{
        Block { index, timestamp, data, hash, prev_hash }
    }

    // Método que retorna o tamanho do bloco
    fn data_size(&self) -> usize{
        self.data.len()
    }

    // Método que retorna o tempo de criação do bloco em segundos
    fn creation_time(&self) -> u64{
        self.timestamp / 1000
    }
}

fn main() {

    let my_block = Block::new(0, 1605991464000, "dados do bloco".to_string(), "hash".to_string(), "hash_anterior".to_string());
    let size = my_block.data_size();
    let time = my_block.creation_time();
    let formatted_string = format!("O tamanho do bloco é {size} e foi criado em {time}");
    
    println!("{}", formatted_string);

}
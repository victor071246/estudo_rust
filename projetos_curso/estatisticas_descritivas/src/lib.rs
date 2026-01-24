use std::collections::HashMap;

// Struct para armazenar dados e fazer os cálculos
pub struct EstatisticasDescritivas{
    numeros: Vec<f64>,
}

impl EstatisticasDescritivas{
    // Construtor para inicializar a struct com um vetor de numeros
    pub fn new(numeros: Vec<f64>) -> Self {
        EstatisticasDescritivas { numeros }
    }

    // Calcular a mediana do vetor de números
    pub fn mediana(&self) -> Result<f64, &'static str>{

        let mut numeros_sorted = self.numeros.clone();
        numeros_sorted.sort();

        if numeros_sorted.is_empty(){
            return Err("O vetor está vazio.");
        }

        let numero_meio = numeros_sorted.len()/2;

        match numeros_sorted.len() % 2{
            0 => {
                let media = EstatisticasDescritivas::media(&vec![numeros_sorted[numero_meio] as f64, numeros_sorted[numero_meio -1] as f64]);
                Ok(media)
            },
            
            _ => Ok(numeros_sorted[numero_meio] as f64),
        }
    }
    // calcular a média do vetor de números em ponto flutuante
    pub fn media(numeros: &Vec<f64>) -> f64{
        let soma: f64 = numeros.iter().sum();
        let quantidade = numeros.len() as f64;
        soma / quantidade

    }

    pub fn moda (&self) -> Vec<f64>{
        let mut frequencia: HashMap<f64, i32> = HashMap::new();

        //Contagem de ocorrências
        for &numero in self.numeros.iter(){
            *frequencia.entry(numero).or_insert(0) += 1;
        }

        // Encontrando o valor de maior frequência
        let max_freq = frequencia.values().cloned().max().unwrap_or(0);

        // Filtrando números de maior frequência
        let moda: Vec<f64> = frequencia.into_iter().filter(|&(|_, freq)|freq == max_freq).map(|(num, _)| num).collect();

        moda
    }
}
fn exist(board: Vec<Vec<char>>, word: String) -> bool{
    let mut board = board;
    let word_chars: Vec<char> = word.chars().collect();

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word_chars: &[char],
        i: usize,
        j: usize,
        k: usize
    ) -> bool{
        // A B C D , E F G H, I J K L => board[0][0] = A
        // board[1][0] = E
        // CACHORRO => k = 0 => "C"
        if k == word_chars.len(){
            return true; // A palavra inteira foi encontrada
        }

        if i >= board[i].len() || j>= board[i].len() || board[i][j] != word_chars[k]{
            return  false; // Fora dos limites ou célula atual não corresponde ao caractere
        }

        // Marca a célula visitada
        let temp = board[i][j];
        board[i][j] = ' ';

        // Explorar todas as direções possíveis
        let result = backtrack(board, word_chars, i+1, j, k+1)
        || backtrack(board, word_chars, i, j+1, k+1)
        || backtrack(board, word_chars, i.saturating_sub(1), j+1, k+1)
        || backtrack(board, word_chars, i, j.saturating_sub(1), k+1);

        // Restaurar a célula
        board[i][j] = temp;

        return result;
    }

    for i in 0..board.len(){
        for j in 0..board[i].len(){
            if backtrack(&mut board, &word_chars, i, j, 0){
                return  true;
            }
        }
    }

    false
}



fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    let word1 = "ABCCED".to_string();
    let word2 = "SEE".to_string();
    let word3 = "ABCB".to_string();

    println!("{}", exist(board.clone(), word1)); // true
    println!("{}", exist(board.clone(), word2)); // true
    println!("{}", exist(board.clone(), word3)); // false
}
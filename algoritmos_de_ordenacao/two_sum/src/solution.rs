pub struct Solution;

impl Solution{
    pub fn two_sums(vetor: Vec<i32>, alvo: i32) -> Vec<i32>{

        println!("vetor = {:?}", vetor);

        for (i, e1) in vetor.iter().enumerate(){
            println!("(i, e1) = {:?}", (i, e1));

            for (j, e2) in vetor.iter().skip(i + 1).enumerate(){
                println!("(j, e2) = {:?}, ", (j, e2));
            }
        }

        vec![]
    }
}
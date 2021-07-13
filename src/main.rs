
use std::convert::TryFrom;
use std::usize;

fn print_hanoi(a: &Vec<u8>, b: &Vec<u8>, c: &Vec<u8>) {
    let max_val = [a.len(), b.len(), c.len()].iter().sum();

    fn get_single_tower_layer(max_val: &usize, opt_val: Option<&u8>) -> String {
        let mut out_vec = vec![' '; max_val * 2];

        if let Some(val) = opt_val {            
            for j in 0..*val {
                out_vec[max_val - 1 + j as usize] = '#';
                out_vec[max_val - 1 - j as usize] = '#';
            }
        }

        return out_vec.into_iter().collect();
    }

    for i in (0..max_val).rev() {        
        println!("| {} | {} | {} |", 
            get_single_tower_layer(&max_val, a.get(i)),
            get_single_tower_layer(&max_val, b.get(i)),
            get_single_tower_layer(&max_val, c.get(i))
        );
    }

    println!("");
}

fn solve_hanoi<'a>(a: &'a mut Vec<u8>, b: &'a mut Vec<u8>, c: &'a mut Vec<u8>, depth: u8,
    deshuffle: for <'b> fn (&'b Vec<u8>, &'b Vec<u8>, &'b Vec<u8>) -> (&'b Vec<u8>, &'b Vec<u8>, &'b Vec<u8>)) {
    if depth == 0 {
        return;
    } else {
        solve_hanoi(a, c, b, depth - 1, |a_, c_, b_| (a_, b_, c_));

        c.push(a.pop().unwrap()); // the actual move

        {
            let (a_, b_, c_) = deshuffle(a, b, c);
            print_hanoi(a_, b_, c_);
        }
        
        solve_hanoi(b, a, c, depth - 1, |b_, a_, c_| (a_, b_, c_));
    }    
}

fn main() {
    let (mut a, mut b, mut c) = (vec![5, 4, 3, 2, 1], vec![], vec![]);
    print_hanoi(&a, &b, &c);
    let depth = u8::try_from(a.len()).unwrap();
    solve_hanoi(&mut a, &mut b, &mut c, depth, |a_, b_, c_| (a_, b_, c_));
}

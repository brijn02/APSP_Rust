use std::collections::HashMap;
use std::fmt::Debug;

use num::Num;

#[derive(Debug)]
pub struct APSPResult<W>
where
    W: Num + Copy + Debug,
{
    pub shortest_paths: HashMap<(usize, usize), W>,
}

impl<W: Num + Copy + Debug> APSPResult<W> {
    pub fn new() -> Self {
        APSPResult {
            shortest_paths: HashMap::new(),
        }
    }

    pub fn add(&mut self, from: usize, to: usize, cost: W) {
        self.shortest_paths.insert((from, to), cost);
    }


    pub fn result_compare(&self, result2: &HashMap<(usize, usize), W>) -> bool {
        let mut returnvalue = true;
     
        for (key, cost1) in &self.shortest_paths {
            match result2.get(key) {
                Some(cost2) => {
                    if cost1 != cost2 {
                        println!("{:?} {:?} {:?}", key, cost1, cost2);
                        returnvalue = false;
                    }
                    

                }
                None => {
                    // If on the main diagional skip, this is always zero
                    if key.0 == key.1 {
                        continue;
                    }

                    
                    returnvalue = false; // Key is missing in result2
                }
            }
        }
        returnvalue
    }

    // Uncommented if you want to use
    // pub fn print_result(&self) {
    //     for key in &self.shortest_paths {
    //         println!("{:?}", key);
    //     }
    // }

    // Uncommented if you want to use
    // pub fn print_result_key(to_print: &HashMap<(usize, usize), W>, &key: &(usize, usize)) {
    //     println!("{:?}", to_print.get(&key));
    // }
}

pub trait APSPAlgorithm<W>
where
    W: Num + Copy + Debug,
{
    fn execute(&mut self);
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool);
    fn get_result(&mut self) -> APSPResult<W>;
}

pub fn calc_mem_exp_cost(mem_byte_size: usize) -> usize {
    let mem_size_word = (mem_byte_size + 31) / 32;
    let mem_cost = (mem_size_word.pow(2) as f64) / 512 as f64 + (3 * mem_size_word) as f64;
    mem_cost.ceil() as usize
}



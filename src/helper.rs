pub struct Helper;

impl Helper {


    pub fn to_word_size(size: usize) -> usize {
        (size + 31) / 32
    }

    pub fn calc_mem_gas_cost(size: usize) -> usize {
        let word_size = Helper::to_word_size(size);
        (3 * word_size) + (word_size.pow(2) / 512)
    }

    pub fn calc_log_gas_cost(size: usize, topic_count: usize, mem_expansion_cost: usize) -> usize {
        let static_gas = 375 * topic_count;
        let mem_gas = 8 * size + mem_expansion_cost;
        static_gas + mem_gas
    }

}

use crate::evm::{EVM, LogEntry};
use crate::memory::MemoryError;
use ethereum_types::{H160, U256};
use tiny_keccak::{Keccak, Hasher};
use crate::helper::Helper;


// ----------- ARITHMETIC -----------
pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a + b);
    evm.gas_decrease(3);
}

pub fn mul(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a * b);
    evm.gas_decrease(5);
}

pub fn sub(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b - a);
    evm.gas_decrease(3);
}

pub fn div(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b / a);
    evm.gas_decrease(5);
}

pub fn sdiv(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b / a);
    evm.gas_decrease(5);
}

pub fn _mod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b % a);
    evm.gas_decrease(5);
}

pub fn smod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b % a);
    evm.gas_decrease(5);
}

pub fn addmod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let c = evm.stack.pop();
    evm.stack.push((b + c) % a);
    evm.gas_decrease(8);
}

pub fn mulmod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let c = evm.stack.pop();
    evm.stack.push((b * c) % a);
    evm.gas_decrease(8);
}

pub fn exp(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b.pow(a));
    evm.gas_decrease(10);
}

// ----------- COMPARISON -----------
pub fn lt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let result = if b < a { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}    

pub fn gt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let result = if b < a { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}

pub fn slt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let result = if b < a { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}

pub fn sgt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let result = if b < a { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}

pub fn eq(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let result = if a == b { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}

pub fn iszero(evm: &mut EVM) {
    let a = evm.stack.pop();
    let result = if a == U256::zero() { 1 } else { 0 };
    evm.stack.push(U256::from(result));
    evm.gas_decrease(3);
}

// ----------- LOGICAL -----------
pub fn and(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b & a);
    evm.gas_decrease(3);
}

pub fn or(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b | a);
    evm.gas_decrease(3);
}

pub fn xor(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b ^ a);
    evm.gas_decrease(3);
}

pub fn not(evm: &mut EVM) {
    let a = evm.stack.pop();
    evm.stack.push(!a);
    evm.gas_decrease(3);
}

// ----------- BITWISE -----------
pub fn byte(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b >> (U256::from(8) * a));
    evm.gas_decrease(3);
}

pub fn shl(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b << a);
    evm.gas_decrease(3);
}

pub fn shr(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b >> a);
    evm.gas_decrease(3);
}

pub fn sar(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b >> a);
    evm.gas_decrease(3);
}

// ----------- PUSH -----------
pub fn push_n(evm: &mut EVM, n: usize) {
    let mut value_bytes = vec![0u8; 32];
    for i in 0..n {
        value_bytes[31 - i] = evm.program[evm.pc + 1 + i];
    }

    evm.stack.push(U256::from_big_endian(&value_bytes));
    evm.pc += n;
    evm.gas_decrease(3);
    
}

// ----------- POP -----------
pub fn pop(evm: &mut EVM) {
    evm.stack.pop();
    evm.gas_decrease(2);
}

// ----------- SWAP -----------
pub fn swap_n(evm: &mut EVM, n: usize) {
    evm.stack.swap(n);
    evm.gas_decrease(3);
}

// ----------- DUPLICATE -----------
pub fn dun_n(evm: &mut EVM, n: usize) {
    evm.stack.dup(n);
    evm.gas_decrease(3);
}

// ----------- MEMORY -----------
// For full 32 bytes
pub fn mstore(evm: &mut EVM) { 
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    let mem_expansion_cost = evm.memory.store(address_as_u64 as usize, &[value]).unwrap();
    evm.gas_decrease(3 + mem_expansion_cost);
}

// For 1 byte = 8 bits
pub fn mstore8(evm: &mut EVM) {
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let byte = (value.low_u64() & 0xFF) as u8;
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    let mem_expansion_cost = evm.memory.store(address_as_u64 as usize, &[U256::from(byte)]).unwrap();
    evm.gas_decrease(3 + mem_expansion_cost);
}

pub fn mload(evm: &mut EVM) {
    let address = evm.stack.pop();
    let result = evm.memory.load(address.as_usize());
    match extract_u256(result) {
        Some(value) => evm.stack.push(value),
        None => {
            // Handle the error case as needed
            panic!("Failed to load value from memory");
        }
    }
    evm.gas_decrease(3);
}

// ----------- STORAGE -----------
pub fn sstore(evm: &mut EVM) {
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    evm.storage.store(address_as_u64 as i32, &[value]);
    evm.gas_decrease(20);
}

pub fn sload(evm: &mut EVM) {
    let address = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // Get the lower 64 bits
    let (warm_access, result) = evm.storage.load(address_as_u64 as i32);

    let value = if let Some(&val) = result.get(0) {
        val
    } else {
        U256::zero()
    };

    evm.stack.push(value);
    let gas_cost = if warm_access { 100 } else { 2100 };
    evm.gas_decrease(gas_cost);
}


pub fn stop(evm: &mut EVM) {
    evm.stop_flag = true;
}

// ----------- TRANSIENT ----------
pub fn tstore(evm: &mut EVM) {
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    evm.transient.store(address_as_u64 as i32, &[value]);
    evm.gas_decrease(100);
}
pub fn tload(evm: &mut EVM) {
    let address = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    let result = evm.transient.load(address_as_u64 as i32);
    let value = if let Some(&val) = result.get(0) {
        val
    } else {
        U256::zero()
    };

    evm.stack.push(value);
    evm.gas_decrease(100);
}

// ----------- HASH -----------
pub fn _keccak256(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    let bytes: Vec<u8> = data.iter().flat_map(|u| {
     let mut buf = [0u8; 32];
     u.to_big_endian(&mut buf);
     buf.to_vec()
 }).collect();
    let hash = _keccak(&bytes);
    evm.stack.push(U256::from_big_endian(&hash));
    evm.gas_decrease(30);
 }

// ----------- LOG -----------
pub fn log0(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    logs_handler(evm, vec![], data.to_vec());
    evm.gas_decrease(375);
}

pub fn log1(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let topic1 = evm.stack.pop();
    println!("Offset is: {:?} and size is: {:?}", offset, size);
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    logs_handler(evm, vec![topic1], data.to_vec());
    evm.gas_decrease(750);
}

pub fn log2(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let topic1 = evm.stack.pop();
    let topic2 = evm.stack.pop();
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    logs_handler(evm, vec![topic1, topic2], data.to_vec());
    evm.gas_decrease(1125);
}

pub fn log3(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let topic1 = evm.stack.pop();
    let topic2 = evm.stack.pop();
    let topic3 = evm.stack.pop();
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    logs_handler(evm, vec![topic1, topic2, topic3], data.to_vec());
    evm.gas_decrease(1500);
}

pub fn log4(evm: &mut EVM) {
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let topic1 = evm.stack.pop();
    let topic2 = evm.stack.pop();
    let topic3 = evm.stack.pop();
    let topic4 = evm.stack.pop();
    let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
    logs_handler(evm, vec![topic1, topic2, topic3, topic4], data.to_vec());
    evm.gas_decrease(1875);
}

// ----------- JUMP ----------
pub fn jump(evm: &mut EVM) {
    let counter = evm.stack.pop().low_u64() as usize;
    if evm.program[counter] != 0x5B {
        panic!("Invalid jump instruction");
    }
    evm.pc = counter;
    evm.gas_decrease(8);
}

pub fn jumpi(evm: &mut EVM) {
    let counter = evm.stack.pop().low_u64() as usize;
    let condition = evm.stack.pop();
    if evm.program[counter] != 0x5B || condition == U256::zero() {
        evm.pc += 1;
    } else {
        evm.pc = counter;
    }
    evm.gas_decrease(10);
}

pub fn pc(evm: &mut EVM) {
    evm.stack.push(U256::from(evm.pc));
    evm.pc += 1;
    evm.gas_decrease(2);
}

pub fn jump_dest(evm: &mut EVM) {
    evm.gas_decrease(1);
}

// ----------- ETHEREUM ---------- [MOCKED]
fn h160_to_u256(address: H160) -> U256 {
    U256::from_big_endian(&address.0)
}
pub fn address(evm: &mut EVM) {
    evm.stack.push(h160_to_u256(evm.sender));
    evm.gas_decrease(2);
}

pub fn balance(evm: &mut EVM) {
    let address = evm.stack.pop();
    let balance = U256::from_big_endian(b"99999"); 
    evm.stack.push(balance);
    evm.gas_decrease(2600); //gas in case of cold address state
}

pub fn origin(evm: &mut EVM) {
    evm.stack.push(h160_to_u256(evm.sender)); // must pass address of account that initiated the txn, not same as sender in case of contracts calling other contracts - sender in an immediate caller
    evm.gas_decrease(2);
}

pub fn caller(evm: &mut EVM) {
    evm.stack.push(h160_to_u256(H160::random())); // caller is the one who invoked current function - random here
    evm.gas_decrease(2);
}

pub fn callvalue(evm: &mut EVM) {
    evm.stack.push(U256::from(evm.value)); // ETH value sent with a call for execution
    evm.gas_decrease(2);
}

pub fn calldataload(evm: &mut EVM) { // reads 32 byte data from calldata starting from offset and push onto stack
    let offset = evm.stack.pop();
    let mut data = Vec::new();
    for i in 0..32 {
        data.push(evm.call_data[offset.low_u64() as usize + i]);
    }
    evm.stack.push(U256::from_big_endian(&data));
    evm.gas_decrease(3);
}

pub fn calldatasize(evm: &mut EVM) { // Get size of call data in current environment
    evm.stack.push(U256::from(evm.call_data.len()));
    evm.gas_decrease(2);
}

pub fn calldatacopy(evm: &mut EVM) { // Copy specified part of input data of this environment to memory
    let dest_offset = evm.stack.pop();
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let mut data = Vec::new();
    data = evm.call_data[offset.low_u64() as usize..offset.low_u64() as usize + size.low_u64() as usize].to_vec();
    evm.memory.store(dest_offset.low_u64() as usize, &[U256::from_big_endian(&data)]);
    evm.gas_decrease(3);
}

pub fn codesize(evm: &mut EVM) { // pushed size of currently running code
    evm.stack.push(U256::from(evm.program.len()));
    evm.gas_decrease(2);
}

pub fn codecopy(evm: &mut EVM) { // Copy running code of this environment to memory
    let dest_offset = evm.stack.pop();
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let mut data = Vec::new();
    data = evm.program[offset.low_u64() as usize..offset.low_u64() as usize + size.low_u64() as usize].to_vec();
    let mem_expansion_cost =evm.memory.store(dest_offset.low_u64() as usize, &[U256::from_big_endian(&data)]).unwrap();
    let dynamic_gas = Helper::to_word_size(size.low_u64() as usize) * 3 + mem_expansion_cost;
    evm.gas_decrease(dynamic_gas);
}

pub fn gasprice(evm: &mut EVM) { // Get price of gas in current environment in Wei
    evm.stack.push(U256::from(0x00)); // random - gas price per unit gas
    evm.gas_decrease(2);
}

pub fn extcodesize(evm: &mut EVM) { // Get size of code at given contractaddress
    let address = evm.stack.pop();
    let address_as_u64 = address.low_u64(); 
    evm.stack.push(U256::from(0)); 
    evm.gas_decrease(2600);
}

pub fn extcodecopy(evm: &mut EVM) {
    let address = evm.stack.pop();
    let dest_offset = evm.stack.pop();
    let offset = evm.stack.pop().as_usize();
    let size = evm.stack.pop().as_usize();
    let code = U256::from(0);  // no external code
    let mem_expansion_cost = evm.memory.store(dest_offset.low_u64() as usize, &[code]);

    evm.gas_decrease(2600);
}

pub fn returndatasize(evm: &mut EVM) { // Get size of return data in current environment from previous call
    evm.stack.push(U256::from(evm.return_data.len()));
    evm.gas_decrease(2);
}

pub fn returndatacopy(evm: &mut EVM) {
    let dest_offset = evm.stack.pop();
    let offset = evm.stack.pop();
    let size = evm.stack.pop();
    let mut data = Vec::new();
    data = evm.return_data[offset.low_u64() as usize..offset.low_u64() as usize + size.low_u64() as usize].to_vec();
    let mem_expansion_cost = evm.memory.store(dest_offset.low_u64() as usize, &[U256::from_big_endian(&data)]).unwrap(); 
    let dynamic_gas = Helper::to_word_size(size.low_u64() as usize) * 3 + mem_expansion_cost;
    evm.gas_decrease(dynamic_gas);
}

pub fn extcodehash(evm: &mut EVM) { // Get hash of code at given contractaddress
    let _ = evm.stack.pop();
    evm.stack.push(U256::from(0)); 
    evm.gas_decrease(2600); // 100 if warm
}

pub fn blockhash(evm: &mut EVM) { // Get hash of one of the 256 most recent block headers
    let _block_no = evm.stack.pop();
    evm.stack.push(U256::zero()); 
    evm.gas_decrease(20);
}

pub fn coinbase(evm: &mut EVM) { // Get address of miner of current block
    evm.stack.push(h160_to_u256(H160::random())); 
    evm.gas_decrease(2);
}


// ----------- CONTRACT -----------
pub fn _return(evm: &mut EVM) { // Return data from current environment
    let mem_dest_offset = evm.stack.pop();
    let size = evm.stack.pop();
    evm.return_data = u256_to_bytes((*evm.memory.access(mem_dest_offset.low_u64() as usize, size.low_u64() as usize).unwrap()).to_vec()).to_vec();
    evm.stop_flag = true
}

pub fn revert(evm: &mut EVM) { // Stops execution and reverts state changes
    let mem_dest_offset = evm.stack.pop();
    let size = evm.stack.pop();
    evm.return_data = u256_to_bytes((*evm.memory.access(mem_dest_offset.low_u64() as usize, size.low_u64() as usize).unwrap()).to_vec()).to_vec();
    evm.revert_flag = true
}



// Helper functions
fn extract_u256(result: Result<&[U256], MemoryError>) -> Option<U256> {
    match result {
        Ok(slice) => {
            // Handle cases where the slice may be empty or has fewer elements than expected
            if let Some(&value) = slice.get(0) {
                Some(value)
            } else {
                // Handle the case where the slice is empty
                None
            }
        }
        Err(_) => {
            // Handle the MemoryError case (e.g., log it or return a default value)
            None
        }
    }
}

fn u8_to_u32_vec(data: &[u8]) -> Vec<u32> {
    data.chunks(4)
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            u32::from_be_bytes(array)
        })
        .collect()
}



fn _keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

fn u256_to_bytes(data: Vec<U256>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for u in data {
        let mut buffer = [0u8; 32];
        u.to_big_endian(&mut buffer);
        bytes.extend_from_slice(&buffer);
    }
    bytes
}

fn logs_handler(evm: &mut EVM, topics: Vec<U256>, data: Vec<U256>) {
    let entry = LogEntry {
        topics: topics,
        data: u256_to_bytes(data),
    };
    evm.logs.push(entry);
}


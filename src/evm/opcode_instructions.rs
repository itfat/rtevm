use crate::evm::{EVM, LogEntry};
use crate::memory::MemoryError;
use ethereum_types::U256;
use tiny_keccak::{Keccak, Hasher};


pub fn stop(evm: &mut EVM) {
    evm.stop_flag = true;
}


pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a + b);
    evm.gas_decrease(3);
}

pub fn pushN(evm: &mut EVM, n: usize) {
    let mut value_bytes = vec![0u8; 32];
    for i in 0..n {
        value_bytes[31 - i] = evm.program[evm.pc + 1 + i];
    }

    evm.stack.push(U256::from_big_endian(&value_bytes));
    evm.pc += n;
    evm.gas_decrease(3);
    
}

pub fn swapN(evm: &mut EVM, n: usize) {
    evm.stack.swap(n);
    evm.gas_decrease(3);
}

pub fn dupN(evm: &mut EVM, n: usize) {
    evm.stack.dup(n);
    evm.gas_decrease(3);
}
// For full 32 bytes
pub fn mstore(evm: &mut EVM) { 
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    evm.memory.store(address_as_u64 as usize, &[value]);
    evm.gas_decrease(3);
}

// For 1 byte = 8 bits
pub fn mstore8(evm: &mut EVM) {
    let address = evm.stack.pop();
    let value = evm.stack.pop();
    let byte = (value.low_u64() & 0xFF) as u8;
    let address_as_u64 = address.low_u64(); // This gets the lower 64 bits
    evm.memory.store(address_as_u64 as usize, &[U256::from(byte)]);
    evm.gas_decrease(3);
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

// pub fn signextend(evm: &mut EVM) {
//     let b = evm.stack.pop();
//     let num = evm.stack.pop();
//     evm.stack.push(extend_sign(num, b));
//     evm.pc += 1;
//     evm.gas_dec(5);
// }

// pub fn extend_sign(num: U256, b: U256) -> U256 {
//     let b = b.as_usize();
//     if b >= 32 {
//         return num;
//     }
//     let bit_index = (b * 8 + 7) as usize;
//     let mask = U256::from(1) << bit_index;
//     let sign_extended_value = if num.bit(bit_index) {
//         num | ((U256::from(1) << ((32 - b - 1) * 8 + 7)) - U256::from(1))
//     } else {
//         num & !(((U256::from(1) << ((32 - b - 1) * 8 + 7)) - U256::from(1)) << 1)
//     };
//     sign_extended_value
// }


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

// ----------- Bitwise -----------
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

// ----------- SHA3 -----------
pub fn _keccak256(evm: &mut EVM) {
   let offset = evm.stack.pop();
   let size = evm.stack.pop();
   println!("Offset is: {:?} and size is: {:?}", offset, size);
   let data = evm.memory.access(offset.low_u64() as usize, size.low_u64() as usize).unwrap();
   println!("Data from memory is: {:?}", data);
   let bytes: Vec<u8> = data.iter().flat_map(|u| {
    let mut buf = [0u8; 32];
    u.to_big_endian(&mut buf);
    buf.to_vec()
}).collect();
   let hash = _keccak(&bytes);
   evm.stack.push(U256::from_big_endian(&hash));
   evm.gas_decrease(30);
}

fn _keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
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

// pub fn address(evm: &mut EVM) {
//     evm.stack.push(evm.sender);
//     evm.pc += 1;
//     evm.gas_decrease(2);
// }

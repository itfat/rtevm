use crate::evm::EVM;
use crate::memory::MemoryError;
use ethereum_types::U256;


pub fn stop(evm: &mut EVM) {
    evm.stop_flag = true;
}


pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a + b);
    evm.gas_decrease(3);
}

pub fn pushN(evm: &mut EVM) {
    let value = evm.program[evm.pc + 1];
    evm.stack.push(U256::from(value));
    evm.pc += 1;
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


fn u8_to_u32_vec(data: &[u8]) -> Vec<u32> {
    data.chunks(4)
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            u32::from_be_bytes(array)
        })
        .collect()
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

// pub fn mul(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(a * b);
//     evm.pc += 1;
//     evm.gas_decrease(5);
// }


// pub fn sub(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b - a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn div(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b / a);
//     evm.pc += 1;
//     evm.gas_decrease(5);
// }

// pub fn sdiv(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b / a);
//     evm.pc += 1;
//     evm.gas_decrease(5);
// }

// pub fn mod(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b % a);
//     evm.pc += 1;
//     evm.gas_decrease(5);
// }

// pub fn smod(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b % a);
//     evm.pc += 1;
//     evm.gas_decrease(5);
// }

// pub fn add_mod(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     let c = evm.stack.pop();
//     evm.stack.push((b + c) % a);
//     evm.pc += 1;
//     evm.gas_decrease(8);
// }

// pub fn mul_mod(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     let c = evm.stack.pop();
//     evm.stack.push((b * c) % a);
//     evm.pc += 1;
//     evm.gas_decrease(8);
// }

// pub fn exp(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b.pow(a as u32));
//     evm.pc += 1;
//     evm.gas_decrease(10);
// }

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


// pub fn lt(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b < a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }    


// pub fn gt(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b > a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn slt(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b < a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn sgt(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b > a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn eq(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b == a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn iszero(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     evm.stack.push((a == 0) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }

// pub fn and(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b & a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn or(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b | a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn xor(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b ^ a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn not(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     evm.stack.push(!a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn byte(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b >> (8 * a));
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn shl(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b << a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn shr(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push(b >> a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn sar(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     let b = evm.stack.pop();
//     evm.stack.push((b as i32 >> a) as usize);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }


// pub fn keccack256(evm: &mut EVM) {
//    offset = evm.stack.pop();
//    size = evm.stack.pop();
//    let data = evm.memory.access(offset, size).unwrap();
//    evm.stack.push(keccak256(data));
//    evm.pc += 1;
//    evm.gas_decrease(30);
// }


// // duplicate top stack item
// pub fn _dup(evm: &mut EVM) {
//     let a = evm.stack.pop();
//     evm.stack.push(a);
//     evm.stack.push(a);
//     evm.pc += 1;
//     evm.gas_decrease(3);
// }

// // swap top of stack with another item given by n
// pub fn swap(evm: &mut EVM, n: u8) {
//     if evm.stack.len() > n+1 as usize {
//         self.stack.swap(0 as usize, n+1 as usize);
//     }

//     self.pc += 1;
//     self.gas_decrease(3);
// }


// pub fn address(evm: &mut EVM) {
//     evm.stack.push(evm.sender);
//     evm.pc += 1;
//     evm.gas_decrease(2);
// }

// pub fn jump(evm: &mut EVM) {
//     let counter = evm.stack.pop();
//     if evm.program[counter] != JUMPDEST {
//         panic!("Invalid jump instruction");
//     }
//     evm.pc = counter;
//     evm.gas_decrease(8);
// }

// pub fn jumpi(evm: &mut EVM) {
//     let counter = evm.stack.pop();
//     let cond = evm.stack.pop();
//     if evm.program[counter] != JUMPDEST || cond == 0 {
//         evm.pc += 1;
//     } else {
//         evm.pc = counter;
//     }
//     evm.gas_decrease(10);
// }

// pub fn pc(evm: &mut EVM) {
//     evm.stack.push(evm.pc);
//     evm.pc += 1;
//     evm.gas_decrease(2);
// }

// pub fn jump_dest(evm: &mut EVM) {
//     evm.pc += 1;
//     evm.gas_decrease(1);
// }

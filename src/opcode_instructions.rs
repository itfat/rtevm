pub fn stop(evm: &mut EVM) {
    evm.stop_flag = true;
}


pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a + b);
    evm.pc += 1;
    evm.gas_decreased(3);
}

pub fn mul(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(a * b);
    evm.pc += 1;
    evm.gas_decreased(5);
}


pub fn sub(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b - a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn div(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b / a);
    evm.pc += 1;
    evm.gas_decreased(5);
}

pub fn sdiv(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b / a);
    evm.pc += 1;
    evm.gas_decreased(5);
}

pub fn mod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b % a);
    evm.pc += 1;
    evm.gas_decreased(5);
}

pub fn smod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b % a);
    evm.pc += 1;
    evm.gas_decreased(5);
}

pub fn add_mod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let c = evm.stack.pop();
    evm.stack.push((b + c) % a);
    evm.pc += 1;
    evm.gas_decreased(8);
}

pub fn mul_mod(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    let c = evm.stack.pop();
    evm.stack.push((b * c) % a);
    evm.pc += 1;
    evm.gas_decreased(8);
}

pub fn exp(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b.pow(a as u32));
    evm.pc += 1;
    evm.gas_decreased(10);
}

pub fn signextend(evm: &mut EVM) {
    let b = evm.stack.pop();
    let num = evm.stack.pop();
    evm.stack.push(extend_sign(num, b));
    evm.pc += 1;
    evm.gas_dec(5);
}

pub fn extend_sign(num: U256, b: U256) -> U256 {
    let b = b.as_usize();
    if b >= 32 {
        return num;
    }
    let bit_index = (b * 8 + 7) as usize;
    let mask = U256::from(1) << bit_index;
    let sign_extended_value = if num.bit(bit_index) {
        num | ((U256::from(1) << ((32 - b - 1) * 8 + 7)) - U256::from(1))
    } else {
        num & !(((U256::from(1) << ((32 - b - 1) * 8 + 7)) - U256::from(1)) << 1)
    };
    sign_extended_value
}


pub fn lt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b < a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}    


pub fn gt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b > a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn slt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b < a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn sgt(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b > a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn eq(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b == a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn iszero(evm: &mut EVM) {
    let a = evm.stack.pop();
    evm.stack.push((a == 0) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}

pub fn and(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b & a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn or(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b | a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn xor(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b ^ a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn not(evm: &mut EVM) {
    let a = evm.stack.pop();
    evm.stack.push(!a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn byte(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b >> (8 * a));
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn shl(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b << a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn shr(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push(b >> a);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn sar(evm: &mut EVM) {
    let a = evm.stack.pop();
    let b = evm.stack.pop();
    evm.stack.push((b as i32 >> a) as usize);
    evm.pc += 1;
    evm.gas_decreased(3);
}


pub fn keccack256(evm: &mut EVM) {
   offset = evm.stack.pop();
   size = evm.stack.pop();
   let data = evm.memory.access(offset, size).unwrap();
   evm.stack.push(keccak256(data));
   evm.pc += 1;
   evm.gas_decreased(30);
}


// duplicate top stack item
pub fn _dup(evm: &mut EVM) {
    let a = evm.stack.pop();
    evm.stack.push(a);
    evm.stack.push(a);
    evm.pc += 1;
    evm.gas_decreased(3);
}

// swap top of stack with another item given by n
pub fn swap(evm: &mut EVM, n: u8) {
    if evm.stack.len() > n+1 as usize {
        self.stack.swap(0 as usize, n+1 as usize);
    }

    self.pc += 1;
    self.gas_decreased(3);
}

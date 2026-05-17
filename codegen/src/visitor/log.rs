//! System instructions

use crate::{masm::MemoryInfo, wasm::ToLSBytes, Error, Function, Result};

impl Function {
    fn frame_base_stack(&self) -> u16 {
        if !self.is_main && self.abi.is_none() {
            1
        } else {
            0
        }
    }

    fn drop_stack_args(&mut self, count: usize) -> Result<()> {
        let available = self.masm.sp().saturating_sub(self.frame_base_stack()) as usize;
        for _ in 0..count.min(available) {
            self.masm._drop()?;
        }

        Ok(())
    }

    fn generic_revert(&mut self, count: usize) -> Result<()> {
        while self.masm.sp() > self.frame_base_stack() {
            self.masm._drop()?;
        }

        self.masm.push(&(count.max(1) * 32).to_ls_bytes())?;
        self.masm._push0()?;
        self.masm._revert()
    }

    /// Parse log data from the bytecode.
    ///
    /// WASM example:
    /// ```text
    /// i32.const 1048576   ;; offset
    /// i32.const 4         ;; 4 bytes
    /// ```
    fn data(&mut self) -> Result<(i32, i32)> {
        let data = self.backtrace.peekn(2).concat();

        // Parse offset.
        //
        // PUSH0 0x5e
        // ..
        // PUSH32 0x8f
        let Some(offset_op) = data.first() else {
            return Err(Error::InvalidDataOffset(0));
        };
        if !(0x5e..0x8f).contains(offset_op) {
            return Err(Error::InvalidDataOffset((*offset_op).into()));
        }

        let offset_len = (*offset_op - 0x5f) as usize;
        if offset_len > 4 || data.len() < offset_len + 1 {
            return Err(Error::InvalidDataOffset((*offset_op).into()));
        }
        tracing::trace!("offset len: {offset_len}");
        let offset = {
            let mut bytes = [0; 4];
            bytes[(4 - offset_len)..].copy_from_slice(&data[1..(offset_len + 1)]);
            bytes.reverse();
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log offset: {:?}", offset);

        // Parse size.
        let Some(size_op) = data.get(offset_len + 1) else {
            return Err(Error::InvalidDataOffset(0));
        };
        if !(0x5e..0x8f).contains(size_op) {
            return Err(Error::InvalidDataOffset((*size_op).into()));
        }
        let size = {
            // TODO: from ls bytes as offset
            let mut bytes = [0; 4];
            let size_bytes = &data[(offset_len + 2)..];
            if size_bytes.len() > bytes.len() {
                return Err(Error::InvalidDataSize(size_bytes.len()));
            }
            bytes[..size_bytes.len()].copy_from_slice(size_bytes);
            i32::from_le_bytes(bytes)
        };

        let buffer: Vec<u8> = self.masm.buffer().into();
        let data_len = data.len();
        self.backtrace.popn(2);
        self.masm.decrement_sp(2)?;
        *self.masm.buffer_mut() = buffer[..(buffer.len() - data_len)].into();

        tracing::debug!("log size: {:?}", size);
        Ok((offset, size))
    }

    /// Log a message with topics.
    pub fn log(&mut self, count: usize) -> Result<()> {
        let (offset, size) = match self.data() {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!("Skipping dynamic log arguments: {error}");
                return self.drop_stack_args(count + 2);
            }
        };
        let data = self.env.data.load(offset, size as usize)?;

        // 1. write data to memory
        let MemoryInfo { offset, size } = self.masm.memory_write_bytes(&data)?;

        // 3. prepare the offset and size of the data.
        self.masm.push(&size.to_ls_bytes())?;
        self.masm.push(&offset)?;

        // 4. run log for the data
        match count {
            0 => self.masm._log0(),
            1 => self.masm._log1(),
            2 => self.masm._log2(),
            3 => self.masm._log3(),
            4 => self.masm._log4(),
            _ => unreachable!("invalid topics"),
        }?;

        Ok(())
    }

    /// Revert with message.
    pub fn revert(&mut self, count: usize) -> Result<()> {
        if self.masm.sp() < (count * 2) as u16 {
            return self.generic_revert(count);
        }

        let mut message = Vec::<Vec<u8>>::default();
        for slot in 0..count {
            let (offset, size) = match self.data() {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!("Using generic revert for dynamic arguments: {error}");
                    return self.generic_revert(count);
                }
            };
            let size = size as usize;
            let data = self.env.data.load(offset, size)?;

            self.masm.push(&data)?;
            if slot == 0 {
                self.masm._push0()?;
            } else {
                self.masm.push(&slot.to_ls_bytes())?;
            }
            self.masm._mstore()?;
            message.push(data);
        }

        tracing::debug!(
            "revert message: {}",
            String::from_utf8_lossy(&message.into_iter().flatten().collect::<Vec<u8>>())
        );

        self.masm.push(&(count * 32).to_ls_bytes())?;
        self.masm._push0()?;

        // 3. run log for the data
        self.masm._revert()?;
        Ok(())
    }
}

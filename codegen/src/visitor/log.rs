//! System instructions

use crate::{masm::MemoryInfo, wasm::ToLSBytes, Error, Function, Result};

impl Function {
    /// Parse log data from the bytecode.
    ///
    /// WASM example:
    /// ```
    /// i32.const 1048576   ;; offset
    /// i32.const 4         ;; 4 bytes
    /// ```
    fn data(&mut self) -> Result<(i32, i32)> {
        let buffer: Vec<u8> = self.masm.buffer().into();

        // Pop offset and size from the bytecode.
        //
        // TODO: backtrace should cross the whole codegen,
        // embed stack operations. (#155)
        let data_len = self.backtrace.popn(2).concat().len();
        self.masm.decrement_sp(2)?;

        let data = &buffer[(buffer.len() - data_len)..];
        *self.masm.buffer_mut() = buffer[..(buffer.len() - data_len)].into();

        // Parse offset.
        //
        // PUSH0 0x5e
        // ..
        // PUSH32 0x8f
        if !(0x5e..0x8f).contains(&data[0]) {
            return Err(Error::InvalidDataOffset(data[0].into()));
        }

        let offset_len = (data[0] - 0x5f) as usize;
        tracing::trace!("offset len: {offset_len}");
        let offset = {
            let mut bytes = [0; 4];
            bytes[(4 - offset_len)..].copy_from_slice(&data[1..(offset_len + 1)]);
            bytes.reverse();
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log offset: {:?}", offset);

        // Parse size.
        if !(0x5e..0x8f).contains(&data[offset_len + 1]) {
            return Err(Error::InvalidDataOffset(data[offset_len + 1].into()));
        }
        let size = {
            // TODO: from ls bytes as offset
            let mut bytes = [0; 4];
            let size_bytes = &data[(offset_len + 2)..];
            bytes[..size_bytes.len()].copy_from_slice(size_bytes);
            i32::from_le_bytes(bytes)
        };

        tracing::debug!("log size: {:?}", size);
        Ok((offset, size))
    }

    /// Log a message with topics.
    pub fn log(&mut self, count: usize) -> Result<()> {
        let mut topics = Vec::<Vec<u8>>::default();
        for topic in (1..=count).rev() {
            let (offset, size) = self.data()?;
            let size = size as usize;
            let data = self.env.data.load(offset, size)?;

            tracing::debug!("log{count} topic{topic}: {:?}", data);
            topics.push(data);
        }

        let name = {
            let (offset, size) = self.data()?;
            let size = size as usize;
            let data = self.env.data.load(offset, size)?;

            tracing::debug!("log1 name: {:?}", data);
            data
        };

        for topic in topics {
            self.masm.push(&topic)?;
        }

        // 1. write data to memory
        let MemoryInfo { offset, size } = self.masm.memory_write_bytes(&name)?;

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
        let mut message = Vec::<Vec<u8>>::default();
        for slot in 0..count {
            let (offset, size) = self.data()?;
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

//! System instructions

use crate::{masm::MemoryInfo, CodeGen, Error, Result, ToLSBytes};

impl CodeGen {
    /// Parse log data from the bytecode.
    fn log_data(&mut self) -> Result<(i32, i32)> {
        let buffer: Vec<u8> = self.masm.buffer().into();

        // Pop offset and size from the bytecode.
        let data_len = self.backtrace.popn(2);
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
        let offset = {
            let mut bytes = [0; 4];
            bytes[..offset_len].copy_from_slice(&data[1..(1 + offset_len)]);
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log0 offset: {:?}", offset);

        // Parse size.
        if !(0x5e..0x8f).contains(&data[offset_len + 1]) {
            return Err(Error::InvalidDataOffset(data[offset_len + 1].into()));
        }
        let size = {
            let mut bytes = [0; 4];
            let size_bytes = &data[(offset_len + 2)..];
            bytes[..size_bytes.len()].copy_from_slice(size_bytes);
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log0 size: {:?}", size);

        Ok((offset, size))
    }

    /// Logs a message without topics.
    pub fn log0(&mut self) -> Result<()> {
        let (offset, size) = self.log_data()?;
        let size = size as usize;
        let data = self
            .dataset
            .get(&offset)
            .ok_or(Error::InvalidDataOffset(offset))?;

        tracing::debug!("log0 data: {:?}", data);
        if data.len() != size {
            return Err(Error::InvalidDataSize(size));
        }

        // 1. write data to memory
        let MemoryInfo { offset, size } = self.masm.memory_write_bytes(data)?;

        // 2. prepare the offset and size of the data.
        self.masm.push(&size.to_ls_bytes())?;
        self.masm.push(&offset)?;

        // 3. run log0 for the data
        self.masm._log0()?;

        Ok(())
    }
}

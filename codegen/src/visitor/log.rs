//! System instructions

use crate::{masm::MemoryInfo, CodeGen, Error, Result, ToLSBytes};

impl CodeGen {
    /// Parse log data from the bytecode.
    fn log_data(&mut self) -> Result<(i32, i32)> {
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
        let offset = {
            let mut bytes = [0; 4];
            bytes[..offset_len].copy_from_slice(&data[1..(1 + offset_len)]);
            i32::from_le_bytes(bytes)
        };
        tracing::debug!("log offset: {:?}", offset);

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

        tracing::debug!("log size: {:?}", size);
        Ok((offset, size))
    }

    /// Log a message with topics.
    pub fn log(&mut self, count: usize) -> Result<()> {
        let mut topics = Vec::<Vec<u8>>::default();
        for topic in (1..=count).rev() {
            let (offset, size) = self.log_data()?;
            let size = size as usize;
            let data = self.dataset.load(offset, size)?;

            tracing::debug!("log{count} topic{topic}: {:?}", data);
            topics.push(data);
        }

        let name = {
            let (offset, size) = self.log_data()?;
            let size = size as usize;
            let data = self.dataset.load(offset, size)?;

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
}

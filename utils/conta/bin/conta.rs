use anyhow::Result;
use clap::Parser;
use conta::Conta;

fn main() -> Result<()> {
    Conta::parse().run()
}

#[macro_use]
extern crate neon;
extern crate rsdb;

mod config;
mod log;
mod tree;

register_module!(m, {
    m.export("Config", config::new)?;
    Ok(())
});

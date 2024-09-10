use std::{env, io};

#[test]
fn test1() -> Result<(), io::Error> {
	println!("current_dir: {:?}", env::current_dir()?);
    println!("current_exe: {:?}",env::current_exe()?);
	Ok(())
}

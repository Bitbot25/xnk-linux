fn main() {
    let res = xnku::syscall_set(60, |context| {
	// do exit
	match xnku::kill(context.pid) {
	    Ok(_) => (),
	    Err(e) => eprintln!("failed to kill process"),
	}
    });
    match res {
	Ok(_) => eprintln!("successfully initialized"),
	Err(_) => eprintln!("failed to register syscall"),
    }
    xnku::;
}

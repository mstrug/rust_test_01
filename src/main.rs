use std::{thread, time, path, fs, io::Write};
use systemstat::{System, Platform};
use chrono::Utc;


fn run_measurements( file: &mut fs::File )
{
	let system = System::new();
	loop 
	{
		match system.cpu_load()
		{
			Ok(dm) => {
				writeln!(file, "\ntimestamp: {}:", Utc::now()).unwrap();
				thread::sleep(time::Duration::from_secs(5));
				match dm.done()
				{
					Ok(v) => {
						for (i, d) in v.iter().enumerate() {
							writeln!(file, "CPU#{}\n\tUser time: {:.2}%\n\tInterrupt time: {:.2}%\n\tSystem time: {:.2}%\n\tIdle time: {:.2}%", 
								i, d.user*100.0, d.interrupt*100.0, d.system*100.0, d.idle*100.0).unwrap();
						}
					},
					Err(x) => {
						println!("Error getting cpu load after some time: {}", x); return;
					}		
				};
			},
			Err(x) => {
				println!("Error getting cpu load: {}", x); return;
			}
		};
	}	
}

fn main()
{
	let fpath = path::Path::new("/tmp/cpu_load.log");

	println!("Measurement started...");
	
	match fs::File::create( &fpath ) {
		Ok(mut file) => {
			println!("Writing to a file: {}", fpath.display());
			run_measurements(&mut file);
		},
		Err(x) => {
			println!("Error creating file: {}, {}", fpath.display(), x);
			return;
		}
	};	
}


use std::{thread, time};
use systemstat::{System, Platform};
use chrono::Utc;



fn main()
{
	let system = System::new();

	println!("Measurement started...");

	loop 
	{
		match system.cpu_load()
		{
			Ok(dm) => {
				println!("timestamp: {}:", Utc::now());
				thread::sleep(time::Duration::from_secs(5));
				match dm.done()
				{
					Ok(v) => {
						for (i, d) in v.iter().enumerate() {
						    println!("CPU#{}\n\tUser time: {:.2}%\n\tNice time: {:.2}%\n\tSystem time: {:.2}%\n\tIdle time: {:.2}%", i, d.user*100.0, d.nice*100.0, d.system*100.0, d.idle*100.0);
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
		println!("");
	}	
}

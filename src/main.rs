use std::{
	collections::HashMap,
	env,
	time::{Duration, Instant},
};

use libc::{_SC_CLK_TCK, sysconf};
use procfs::{ProcError, process::Process};

const TIME_TO_MEASURE: u64 = 100;

fn main() {
	if let Ok((time, res)) = real_main() {
		let tck = unsafe { sysconf(_SC_CLK_TCK) } as f32;
		println!("{: >4}", format!("{:.1}", 1000.0 * res as f32 / tck / time as f32));
	} else {
		println!("00.0");
	}
}

/// Returns: measurement time in milliseconds, total utime + stime
fn real_main() -> Result<(usize, u64), ProcError> {
	let pid: i32 = env::args().skip(1).next().unwrap().parse().unwrap();
	let p = Process::new(pid)?;
	let mut all_pids = vec![p.pid];
	all_pids.extend_from_slice(&get_all_pids(&p)?);
	// get current load
	let start = Instant::now();
	let mut total_sched = HashMap::new();
	for pid in &all_pids {
		let Ok(p) = Process::new(*pid) else {
			continue;
		};
		let Ok(s) = p.stat() else {
			continue;
		};
		total_sched.insert(p.pid, s.utime + s.stime);
	}
	std::thread::sleep(Duration::from_millis(TIME_TO_MEASURE));
	let end = Instant::now();
	let mut total_sched_2 = HashMap::new();
	for pid in &all_pids {
		let Ok(p) = Process::new(*pid) else {
			continue;
		};
		let Ok(s) = p.stat() else {
			continue;
		};
		total_sched_2.insert(p.pid, s.utime + s.stime);
	}
	let mut total = 0;
	for (pid, total1) in total_sched {
		if let Some(total2) = total_sched_2.get(&pid) {
			total += total2 - total1;
		}
	}
	Ok((end.duration_since(start).as_millis() as _, total))
}

fn get_all_pids(p: &Process) -> Result<Vec<i32>, ProcError> {
	let mut all = vec![];
	for child in p.tasks()? {
		let Ok(child) = child else {
			continue;
		};
		for child in child.children()? {
			let Ok(p) = Process::new(child as _) else {
				continue;
			};
			all.push(child as i32);
			let remainder = get_all_pids(&p).unwrap_or_default();
			all.extend_from_slice(&remainder);
		}
	}
	Ok(all)
}

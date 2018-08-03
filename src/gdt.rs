use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096;
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			// unsafe mut pointer because we don't have proper memory management yet
			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			let stack_end = stack_start + STACK_SIZE;
			stack_end
		};
		tss
	};
}

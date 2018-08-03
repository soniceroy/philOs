https://os.phil-opp.com/

System prereqs: 
  'cargo install cargo-xbuild'
  'rustup component add rust-src'
  'cargo install bootimage --version "^0.5.0"'
  
To build: 'bootimage build'
To run all tests: 'bootimage test'
To run bins (using qemu) std out: 
	'bootimage run -bin $TESTBINNAME -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none'
To run (using qumu) output file:
	'bootimage run -bin $TESTBINNAME -- -serial file:$FILENAME -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none' 
--'isa-debub-exit' device is a simple way to exit Qemu.
--'iobase' specifies the port where the device lives (0xf4 being
   a generally unused port on the x86)
--'iosize' specifies port size (0x04=4bytes)
--guest writes a value to the exit device, and Qemu
   responds by exiting with exit status (value << 1) | 1.
To write to usb stick for use on a real machine (assuming in project directory):
    where sdX is the device name:
        'dd if=target/x86_64-phil_os/debug/bootimage.bin of=/dev/sdX && sync'


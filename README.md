https://os.phil-opp.com/

System prereqs: 
  'cargo install cargo-xbuild'
  'rustup component add rust-src'
  'cargo install bootimage --version "^0.5.0"'
  
To build: 'bootimage build'
To run (using qemu): 'bootimage run'

To write to usb stick for use on a real machine (assuming in project directory):
    where sdX is the device name:
        'dd if=target/x86_64-phil_os/debug/bootimage.bin of=/dev/sdX && sync'
    

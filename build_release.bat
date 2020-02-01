cls
rd /S /Q target
cargo rustc --target=thumbv7m-none-eabi --release -- -C link-arg=-Tlink.x -C link-arg=-nostartfiles -C linker=arm-none-eabi-gcc -C link-args="-Wl,-Map=stm32f103.map" 
ren target\thumbv7m-none-eabi\release\stm32f103 *.elf

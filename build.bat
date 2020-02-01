cls
rd /S /Q target
cargo rustc --target=thumbv7m-none-eabi -- -C link-arg=-Tlink.x -C link-arg=-nostartfiles -C linker=arm-none-eabi-gcc -C link-args="-Wl,-Map=stm32f103.map, -g"
ren target\thumbv7m-none-eabi\debug\stm32f103 *.elf
rem arm-none-eabi-objdump -d target\thumbv6m-none-eabi\debug\hello_arm.elf
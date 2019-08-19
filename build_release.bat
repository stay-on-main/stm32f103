cls
rd /S /Q target
cargo rustc --target thumbv6m-none-eabi --release -- -C link-arg=-Tlink.x -C link-arg=-nostartfiles
ren target\thumbv6m-none-eabi\release\hello_arm *.elf
arm-none-eabi-objdump -d target\thumbv6m-none-eabi\release\hello_arm.elf
mode = debug
arm9_path = arm9/target/sd3_arm9/$(mode)/sd3_arm9
arm11_path = arm11/target/sd3_arm11/$(mode)/sd3_arm11
device_label = 3DS
firm_path = sd3.firm
firmtool = rust

ifeq ($(mode),release)
  cargo_flags += --release
endif

firm: $(firmtool)

rust: arm9 arm11
	cargo run --manifest-path firmtool/Cargo.toml $(firm_path) $(arm9_path) $(arm11_path)

py: arm9 arm11
	firmtool build $(firm_path) -D $(arm9_path) $(arm11_path) -C NDMA XDMA

arm9:
	cd arm9; cargo xbuild $(cargo_flags)

arm11:
	cd arm11; cargo xbuild $(cargo_flags)

parse: firm
	firmtool parse $(firm_path)

vis: firm
	ksv $(firm_path) formats/firm.ksy

clean:
	cd arm9; cargo clean
	cd arm11; cargo clean
	cd firmtool; cargo clean

deploy: firm
	udisksctl mount -b "/dev/disk/by-label/$(device_label)"
	cp $(firm_path) "/var/run/media/$(USER)/$(device_label)/boot.firm"
	umount "/var/run/media/$(USER)/$(device_label)"
	sync

.PHONY: firm rust py arm9 arm11 parse vis clean deploy

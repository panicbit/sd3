arm9_path = arm9/target/sd3_arm9/debug/sd3_arm9
arm11_path = arm11/target/sd3_arm11/debug/sd3_arm11
device_label = 3DS
firm_path = sd3.firm

firm: arm9 arm11
	firmtool build $(firm_path) -D $(arm9_path) $(arm11_path) -C NDMA XDMA

arm11:
	cd arm11; cargo xbuild

arm9:
	cd arm9; cargo xbuild

parse: firm
	firmtool parse $(firm_path)

vis:
	ksv $(firm_path) formats/firm.ksy

clean:
	cd arm9; cargo clean
	cd arm11; cargo clean

deploy: firm
	udisksctl mount -b "/dev/disk/by-label/$(device_label)"
	cp $(firm_path) "/var/run/media/$(USER)/$(device_label)/boot.firm"
	umount "/var/run/media/$(USER)/$(device_label)"
	sync

.PHONY: firm arm9 arm11

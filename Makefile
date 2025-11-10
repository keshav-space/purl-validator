build-fst:
	cargo run --bin fst_builder


clean:
	cargo clean
	rm -f purls.fst
	rm -rf target

.PHONY:  build-fst clean
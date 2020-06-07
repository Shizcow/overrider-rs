# Compiles docs
docs:
	@echo ":::: Cleaning up before documenting..."
	cargo clean
	cargo update
	@echo ":::: Generating docs..."
	cargo doc -p overrider -p overrider_build --no-deps
	@echo "Docs are available at:"
	@echo "    target/doc/overrider/index.html"
	@echo "    target/doc/overrider_build/index.html"

# Publishes everything to crates.io
publish:
	cd overrider && cargo publish
	cd overrider_build && cargo publish

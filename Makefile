#
# Makefile
# @author Evgeny Ukhanov <mrlsd@ya.ru>
#

.PHONY: run, build, release

default: build

run:
	@echo Running...
	@cargo run
	@echo Done.

build:
	@echo Build debug version...
	@cargo build
	@echo Done.

release:
	@echo Build release...
	@cargo build --release
	@echo Done.

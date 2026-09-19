SHELL := /bin/bash
CARGO ?= $(HOME)/.cargo/bin/cargo
CHROME ?= /Applications/Google Chrome.app/Contents/MacOS/Google Chrome
ROOT := $(CURDIR)

.PHONY: all build pdf serve clean

# Full pipeline: resume.yaml -> index.html + print.html -> resume.pdf
all: pdf

# Render index.html and print.html from resume.yaml
build:
	cd builder && "$(CARGO)" run --release --quiet

# Print print.html to resume.pdf with headless Chrome
pdf: build
	"$(CHROME)" --headless --disable-gpu --no-pdf-header-footer \
	  --print-to-pdf="$(ROOT)/resume.pdf" "file://$(ROOT)/print.html"

# Preview the site locally
serve:
	python3 -m http.server 3000 --bind 127.0.0.1

clean:
	cd builder && "$(CARGO)" clean

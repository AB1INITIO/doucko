.PHONY: build serve clean

build:
	@bash scripts/build.sh

serve: build
	@echo "Serving at http://localhost:8000 (watching for changes...)"
	@trap 'kill 0' EXIT INT TERM; \
	 bash scripts/watch.sh & \
	 cd _site && python3 -m http.server 8000

clean:
	@rm -rf _site
	@echo "Cleaned."

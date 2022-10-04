all:
	rm -rf pkg
	wasm-pack build --target=no-modules
	cp manifest.json pkg/manifest.json
	cp -a js/. pkg/
	printf "async function run() {\n\tawait wasm_bindgen(browser.runtime.getURL('trimps_tracker_bg.wasm'));\n}\n\nrun();" >> pkg/run_wasm.js
	for i in pkg/*.js; do \
		[ -f "$${i}" ] || break ; \
		minify -o $${i} $${i} ; \
	done
	for i in pkg/*.wasm; do \
		[ -f "$${i}" ] || break ; \
		wasm-opt -Os $${i} -o $${i} ; \
	done
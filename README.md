# rust_for_frontend
- prototype of frontend website using:
  - rust (compiles into WASM)
  - html
  - css

## Setup  
	- Install rustup
	- `rustup default stable` to download the latest stable release of Rust and set it as your default toolchain.  
	- ``rustup target add wasm32-unknown-unknown ``
  - ``cargo install trunk ``
  - ``export PATH="$HOME/.cargo/bin:$PATH" ``
  - ``trunk serve`` 
		
## Docker Image (Building and running)
	- `trunk build` 
	- `docker build . -t html-image`
	- `docker run -d -p 8080:80 html-image` 

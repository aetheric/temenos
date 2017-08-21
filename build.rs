extern crate protoc_rust;

fn main() {

	// Compile all the protocol buffers.
	protoc_rust::run(protoc_rust::Args {
		out_dir:  "src",
		input:    &[ "res/messages.proto" ],
		includes: &[ "res" ],
	}).expect("protoc");

}

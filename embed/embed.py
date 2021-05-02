from ctypes import cdll

lib = cdll.LoadLibrary("/Users/kuromt/git/wasm-learning/embed/target/release/libembed.dylib")

lib.process()

print("done!")
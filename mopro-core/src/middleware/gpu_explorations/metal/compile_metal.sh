# every kernel is self contained i.e. is its own crate. Simply `cd` into a kernel's directory and run
# the following, which compiles our shader to an intermediate representation using the metal utility
xcrun -sdk macosx metal -c ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.metal -o ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air

# next, compile the .air file to generate a .metallib file - which I believe is LLVM IR (need confirmation)
xcrun -sdk macosx metallib ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air -o ./mopro-core/src/middleware/gpu_explorations/metal/shader/msm.metallib

# finally, clean the redundant .air file
rm -f ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air
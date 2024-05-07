# every kernel is self contained i.e. is its own crate. Simply `cd` into a kernel's directory and run
# the following, which compiles our shader to an intermediate representation using the metal utility
xcrun -sdk macosx metal -c ./shader/all.metal -o ./shader/all.air

# next, compile the .air file to generate a .metallib file - which I believe is LLVM IR (need confirmation)
xcrun -sdk macosx metallib ./shader/all.air -o ./shader/msm.metallib

# finally, clean the redundant .air file
rm -f ./shader/all.air
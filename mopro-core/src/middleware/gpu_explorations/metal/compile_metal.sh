# every kernel is self contained i.e. is its own crate. Simply `cd` into a kernel's directory and run
# the following, which compiles our shader to an intermediate representation using the metal utility

# input a cmd parameter that is able to change the target of macosx and iphoneos
# ios: compileing metal with `iphoneos` target
# mac: compiling metal with `macosx` target
ARG=$1
if [ $ARG == "ios" ]; then
    TARGET="iphoneos"
elif [ $ARG == "mac" ]; then
    TARGET="macosx"
else
    echo "Unknown target: $ARG"
    echo "Please enter either 'ios' or 'mac' as parameter to compile"
    exit 1
fi

xcrun -sdk $TARGET metal -c ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.metal -o ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air

# next, compile the .air file to generate a .metallib file - which I believe is LLVM IR (need confirmation)
xcrun -sdk $TARGET metallib ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air -o ./mopro-core/src/middleware/gpu_explorations/metal/shader/msm.metallib

# finally, clean the redundant .air file
rm -f ./mopro-core/src/middleware/gpu_explorations/metal/shader/all.air
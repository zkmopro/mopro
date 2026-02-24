pragma circom 2.1.9;

template Demo() {
    signal input in;
    signal output out;

    signal x <-- 1;
    out <== in * x;
}
component main = Demo();
pragma circom 2.2.3;

include "circomlib/circuits/multiplexer.circom";

template Demo {
    signal input in;
    signal output out;

    component dotCheck = Multiplexer(1,2);
    dotCheck.inp[0][0] <== 0;
    dotCheck.inp[1][0] <== in;
    dotCheck.sel <== 1;
    dotCheck.out[0] ==> out;
}
component main = Demo();
from sympy import mod_inverse

N = 21888242871839275222246405745257275088696311157297823662689037894645226208583
mod = 1 << 32

# Step 1: N^-1 mod 2^32
try:
    N_inv = mod_inverse(N, mod)
except ValueError as e:
    print(e)
    N_inv = None

# Step 2: Compute -N^-1 mod 2^32
if N_inv is not None:
    result = mod - N_inv

# Step 3: Make sure N_inv is computed correctly
if N_inv is not None:
    # compute N * -N^-1 mod 2^32 = -1
    print("Check that N * N^-1 mod 2^32:", (N * result) % mod)
    assert (N * result) % mod == mod - 1

# Step 4: Convert the result to a hexadecimal string
hex_str = hex(result)[2:]  # Removing the '0x' prefix

# Step 5: Ensure the hex string length is a multiple of 8 by padding with leading zeros if necessary
if len(hex_str) % 8 != 0:
    hex_str = hex_str.zfill((len(hex_str) // 8 + 1) * 8)

# Step 6: Split the hex string into chunks of 8 characters (Big Endian order)
limbs = [hex_str[i:i+8].upper() for i in range(0, len(hex_str), 8)]

# Print the results in Big Endian order
print(f"Result in Hexadecimal: 0x{hex_str.upper()}FFFFFFFF")
print(f"Result in Decimal: {result}")
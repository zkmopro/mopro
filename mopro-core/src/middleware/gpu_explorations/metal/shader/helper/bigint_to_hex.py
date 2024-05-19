# Big integer to convert
big_int = 21888242871839275222246405745257275088696311157297823662689037894645226208583

# Step 1: Convert the big integer to a hexadecimal string
hex_str = hex(big_int)[2:]  # Removing the '0x' prefix

# Step 2: Ensure the hex string length is a multiple of 8 by padding with leading zeros if necessary
if len(hex_str) % 8 != 0:
    hex_str = hex_str.zfill((len(hex_str) // 8 + 1) * 8)

# Step 3: Split the hex string into chunks of 8 characters (Big Endian order)
limbs = [hex_str[i:i+8].upper() for i in range(0, len(hex_str), 8)]

# Print the results in Big Endian order
print("Decimal Integer:", big_int)
print("Hexadecimal String:", hex_str)
print("32-bit unsigned integer limbs in hex format (Big Endian):")

# \n for every two limbs
for i in range(0, len(limbs), 2):
    print("0x" + limbs[i] + ", 0x" + limbs[i+1] + ",")

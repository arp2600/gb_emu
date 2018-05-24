#!/usr/bin/env python3
# Usage:
#     ./opcode_examiner.py 7a 7b 7c 7d 7e 7f
import sys


def main():
    opcodes = sys.argv[1:]
    opcodes = [int(x, 16) for x in opcodes]
    opcodes.sort()
    bincodes = ['{:08b}'.format(x) for x in opcodes]
    bincodes = [list(x) for x in bincodes]

    master_code = bincodes[0]
    for bc, oc in zip(bincodes, opcodes):
        for i in range(0, len(master_code)):
            if master_code[i] != bc[i]:
                master_code[i] = 'x'
        print("0x{:02x} 0b{:08b}".format(oc, oc))

    out = ''
    was_x = master_code[0] == 'x'
    for i in range(0, len(master_code)):
        is_x = master_code[i] == 'x'
        if is_x != was_x:
            out += '_'
            was_x = is_x
        out += master_code[i]
    print('opcode format = {}'.format(out))


if __name__ == '__main__':
    main()

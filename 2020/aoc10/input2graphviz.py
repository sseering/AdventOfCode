#!/usr/bin/env python3

import sys


def main() -> None:
    if len(sys.argv) != 2:
        print('Error: need input filename as commandline arg. Exiting.', file=sys.stderr)
        sys.exit(1)

    with open(sys.argv[1], mode='rt', encoding='utf8') as in_f:
        adapters = set(int(_) for _ in in_f.readlines())

    computer = max(adapters) + 3
    power_socket = 0

    with open('/tmp/out.gv', mode='wt', encoding='utf8') as out_f:
        out_f.write('digraph G {rankdir=LR;')
        for a in adapters:
            for jolt_in in [a-3, a-2, a-1]:
                if jolt_in == power_socket:
                    out_f.write('power_socket -> A{0};'.format(a))
                elif jolt_in in adapters:
                    out_f.write('A{0} -> A{1};'.format(jolt_in, a))

        for jolt_in in [computer-3, computer-2, computer-1]:
            if jolt_in == power_socket:
                out_f.write('power_socket -> computer;')
            elif jolt_in in adapters:
                out_f.write('A{0} -> computer;'.format(jolt_in))

        out_f.write('}')

    print('wrote /tmp/out.gv')
    print('convert with dot -Tsvg /tmp/out.gv > /tmp/out.svg')
    print('done')


if __name__ == "__main__":
    main()

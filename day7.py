import re

def emulate(lines, overrides=None):
    signals = {str(i): i for i in range(16)}
    gates = []
    overrides = overrides or []

    for l in lines:
        sig = re.match("(\d+) -> (\w)+", l)
        nop = re.match("(\w+) -> (\w+)", l)
        no = re.match("NOT (\w+) -> (\w+)", l)
        op = re.match("(\w+) (\w+) (\w+) -> (\w+)", l)

        if sig is not None:
            signals[sig.group(2)] = int(sig.group(1))
        elif nop is not None:
            gates.append(((nop.group(1),), "NOP", nop.group(2)))
        elif no is not None:
            gates.append(((no.group(1),), "NOT", no.group(2)))
        elif op is not None:
            gates.append(((op.group(1), op.group(3)), op.group(2), op.group(4)))
        else:
            print(l)
            raise Exception()

    for wire, value in overrides:
        signals[wire] = value

    while gates:
        inp, op, out = gates.pop(0)

        if not all([i in signals for i in inp]):
            gates.append((inp, op, out))
            continue

        if op == "NOP":
            signals[out] = signals[inp[0]]
        elif op == "NOT":
            signals[out] = ~signals[inp[0]]
        elif op == "AND":
            signals[out] = signals[inp[0]] & signals[inp[1]]
        elif op == "OR":
            signals[out] = signals[inp[0]] | signals[inp[1]]
        elif op == "LSHIFT":
            signals[out] = signals[inp[0]] << signals[inp[1]]
        elif op == "RSHIFT":
            signals[out] = signals[inp[0]] >> signals[inp[1]]

    return signals

with open("day7.in") as fin:
    circuit = [l.strip() for l in fin]

sigs = emulate(circuit)
print(sigs["a"])
sigs = emulate(circuit, [("b", sigs["a"])])
print(sigs["a"])

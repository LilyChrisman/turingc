# TuringC
## Made as a side project for EWU CSCD 420-040

Compiles formal, deterministic, single tape turing machine code.
The compiled turing machine is ran on a single line input to standard in, which represents the initial tape state.
This program will obviously not be able to run every turing machine, unless your RAM is a real infinite tape.

example script:
```
Initial = q_0
Final = q_f
#
First 2 lines must be formed as above
Multiple final states are not currently supported because I have not needed them
#

# Tally mark addition with the form addition(1^n+1^m) = 1^(n+m) #

# Final state, all 1's have been added to the right side #
δ(q_0, +) = (q_f, ☐, L)

# Remove the leftmost one and travel to the far right side #
δ(q_0, 1) = (q_1, ☐, R)
δ(q_1, 1) = (q_1, 1, R)

δ(q_1, +) = (q_1, +, R)

# Traverse left to find another 1 #
δ(q_1, ☐) = (q_2, 1, L)
δ(q_2, 1) = (q_2, 1, L)
δ(q_2, +) = (q_2, +, L)

# Restart the process #
δ(q_2, ☐) = (q_0, ☐, R)
```

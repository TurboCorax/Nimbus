# Netlist Implementation

## Overview

The general syntax basically refers to the implementation of LTspice netlists. The netlist is a text file that
contains a description of the circuit. It is used to define the components and their connections in the
circuit. The netlist is read by the simulator to create the circuit.

Referenced
Rules: [General Structure and Conventions](https://ltwiki.org/LTspiceHelp/LTspiceHelp/A_General_Structure_and_Conventions.htm)

Unsupported features will be listed in this document. Otherwise, the netlist implementation will be the same
as LTspice. (At least, it should be.)

If there is some extra feature that is not in LTspice, it will be mentioned in the documentation.

## Unsupported Features

- numbers written in the form 6K34 to mean 6.34K

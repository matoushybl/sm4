# Software



## Known issues

* the main loop cannot run @ 1 kHz as the readings of the encoder timer are zero, this is likely a software bug. The current workaround is to lower the control frequency to 100 Hz.
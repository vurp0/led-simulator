#!/usr/bin/env python3

import sys
import os
import math
import time
import colorsys

ser = os.fdopen(sys.stdout.fileno(), 'wb')


while 1:
  for x in range(50):
    color = colorsys.hsv_to_rgb((time.time()*100+x/50)%1, 1, 1)
    ser.write(bytes([x,
      int(color[0]*127),int(color[1]*127),int(color[2]*127)]))
    ser.flush()

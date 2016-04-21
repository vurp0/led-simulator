#!/usr/bin/python3

#import serial
import random
import time
import math
import sys
import os
import pytweening
import colorsys

#ser = serial.Serial('/dev/ttyUSB0', 9600)
ser = os.fdopen(sys.stdout.fileno(), 'wb')

def clamp(x, min, max):
  if x>max: return max
  if x<min: return min
  else: return x

startTime = time.time()
startIndexes = list(range(50))
random.shuffle(startIndexes)

while 1:
  #data = []
  #now = time.time()
  
  for i in range(50):
 
    fade = pytweening.easeInOutBounce(clamp((time.time()-(startTime+0.0006*startIndexes[i]))*20, 0, 1))
    
    red =   int(127*(math.sin(time.time()*1000*0.143+(i*0.24))*0.5+0.5)*fade);
    green = int(127*(math.sin(time.time()*1000*0.264+(i*0.43))*0.5+0.5)*fade);
    blue =  int(127*(math.sin(time.time()*1000*0.403+(i*0.41))*0.5+0.5)*fade);
    """color = colorsys.hsv_to_rgb((time.time()*50+i/50)%1, 1, 1)
    red =  int(127*(color[0]*fade))
    green = int(127*(color[1]*fade))
    blue = int(127*(color[2]*fade))"""

    data = [i|0b10000000, clamp(red, 0, 127), clamp(green, 0, 127), clamp(blue, 0, 127)]
    ser.write(bytes(data))
    ser.flush()
  #time.sleep(0.1)

ser.close()


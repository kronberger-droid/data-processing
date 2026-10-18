"""Usage: play.py -t # [-h]
            -s sin|ramp|noise play either a sinus or a ramp or noise
            -t # Time in seconds to be played (default: 5)
            -f # Frequency in Hz (default: 440)
            -d # Frequency range for ramp (default: 8000)
            -a # Amplitude (default: 30000)
            -h: Print this help message
  RESULT:
   Play a sound t seconds at your default sound card.
   With a sample rate of 22050 Hz in format S16LE (16 bit int little endian)  
"""
import sys
import time
import getopt
import alsaaudio
from math import *
from getopt import *
from random import *

# -------------------------------------
def make_ramp(f0=30.,df=2000., ampl=30000., rate=22050,length=5):
  """ Generate a frequency ramp
      changing frequency continous from f0 ... f0+df
      f0: start frequency in Hz
      df: frequency change
      ampl: amplitude max. value depends on output format
            S16LE: signed 16 bit
                   a(max): 2**(16-1)-1
                   min-value: -2**(16-1)-1
      rate: sample rate (samples/s)
      length: duration of sound in sec
      return: character buffer with PCM pattern 
  """
  # Zu Programmieren
  return b""
# -------------------------------------
def make_sin(f0=1000.,ampl=30000,rate=22050,length=5.):
  """ Fill the byte-buffer with a sinodal wave
      f0: frequency in Hz
      ampl: amplitude
      rate: sample rate (samples/s)
      length: duration of sound in sec
      return: character buffer with PCM pattern 
  """
  a = 2. * pi * f0/rate              
  n = int(rate * length)
  wav=b''
  for i in range(0, n):
    f = int(ampl*sin(a*i))
    wav += bytes([f & 0x00FF, (f & 0xFF00) >> 8])
  return wav
# ------------------------------------------
def make_noise(ampl=30000,rate=22050,length=5.):
    """ Generate a noise signal
          parameters:
            amplitude, sample rate, duration
       return: character buffer with PCM pattern 
   """
    return b""
  # Zu Programmieren 
# ------------------------------------------
def make_triangle(ampl=30000,rate=22050,length=5.):
    """ Generate a triangle or sawtooth signal
        parameters:
            amplitude, sample rate, duration
       return: character buffer with PCM pattern 
   """
    return b""
  # Zu Programmieren 
# ------------------------------------------
def make_rectangle(ampl=30000,rate=22050,length=5.):
    """ Generate a rectangle signal
        parameters:
            amplitude, sample rate, duration
       return: character buffer with PCM pattern 
   """
    return b""
  # Zu Programmieren 
# ------------------------------------------

if __name__ == '__main__':
  opt=getopt(sys.argv[1:], "s:f:d:t:a:h")

  rate = 22050
  crd = 'default'
  sound= 'sin'
  freq=440.
  df=8000.
  tr=5.
  ampl=30000
  for o in opt[0]:
     if o[0]=='-t': tr=float(o[1])
     if o[0]=='-s': sound=o[1]
     if o[0]=='-f': freq=float(o[1])
     if o[0]=='-d': df=float(o[1])
     if o[0]=='-a': ampl=float(o[1])
     if o[0]=='-h': print(__doc__); sys.exit(0)

  type_d={"sin": {"func": make_sin, "args":{"f0":freq,"ampl":ampl,"rate":22050,"length":tr}},
         "ramp": {"func": make_ramp,"args":{"f0":freq,"df":df,"ampl":ampl,"rate":22050,"length":tr}},
        "noise": {"func": make_noise,"args":{"ampl":ampl,"rate":22050,"length":tr}}}


  wave=type_d[sound]["func"](**type_d[sound]["args"])

  # Open the device in playback mode. 
  out = alsaaudio.PCM(alsaaudio.PCM_PLAYBACK)#, card=crd)
  # Set attributes: Mono, 44100 Hz, 16 bit little endian frames
  out.setchannels(1)
  out.setrate(rate)
  out.setformat(alsaaudio.PCM_FORMAT_S16_LE)
  out.setperiodsize(160)

  i = 0
  l= len(wave)
  while i < l:
       i+=2*out.write(wave[i:i+320])









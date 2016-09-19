#! /bin/bash

# Downloads dependencies on Ubunutu 16.04
# Other Ubuntu versions will likely work but things may have been renamed.
# Untested (for now)

sudo apt-get install -y build-essential git cmake libfreeimage-dev cmake-curses-gui \
  libglew-dev libatlas3-base libatlas-dev libfftw3-dev liblapacke-dev \
  libboost-all-dev opencl-headers libglew-dev libglewmx-dev libglfw3-dev

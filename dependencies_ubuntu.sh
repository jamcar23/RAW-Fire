#! /bin/bash

# Copyright (c) 2016 James Carroll
# This file is part of RAW Fire.
#
# RAW Fire is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# RAW Fire is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with RAW Fire.  If not, see <http://www.gnu.org/licenses/>.

# Downloads dependencies on Ubunutu 16.04
# Other Ubuntu versions will likely work but things may have been renamed.
# Untested (for now)

git submodule update --init --recursive

sudo apt-get install -y build-essential git cmake libfreeimage-dev cmake-curses-gui \
  libglew-dev libatlas3-base libatlas-dev libfftw3-dev liblapacke-dev \
  libboost-all-dev opencl-headers libglew-dev libglewmx-dev libglfw3-dev

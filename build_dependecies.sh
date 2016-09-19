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

HAS_CUDA=OFF
HAS_OPENCL=ON

root_dir="./"
build="/build/"

# Arrayfire
af_path="./arrayfire-rust/arrayfire$build"
mkdir $af_path && cd $af_path
cmake .. -DCMAKE_BUILD_TYPE=Release -DBUILD_CUDA=$HAS_CUDA -DBUILD_OPENCL=$HAS_OPENCL
make -j8
sudo make install
cd $root_dir

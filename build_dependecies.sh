#! /bin/bash

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

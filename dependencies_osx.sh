#! /bin/bash

# Download submodules
git submodule update --init --recursive

# Install OSX dependencies via brew
brew tap homebrew/versions
brew install cmake freeimage fftw boost glew glfw3
root_dir="./"

# Arrayfire
af_path="./arrayfire-rust/arrayfire"
af_build="$af_path/build/"
cd $af_path
mkdir $af_build
cmake $af_path -DCMAKE_BUILD_TYPE=Release -DBUILD_CUDA=OFF -DBUILD_OPENCL=ON
make -j8
sudo make install
cd $root_dir

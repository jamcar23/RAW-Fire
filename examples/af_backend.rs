/**
* Copyright (c) 2016 James Carroll
* af_backend.rs is part of RAW Fire.
*
* RAW Fire is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* RAW Fire is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with RAW Fire.  If not, see <http://www.gnu.org/licenses/>.
*/

// File to test the different backends of ArrayFire

extern crate arrayfire as af;

use af::*;

#[cfg(op_assign)]
fn helper(dims: Dim4) {
    let mut a = randu::<f32>(dims);
    let b = randu::<f32>(dims);
    print(&a);
    print(&b);
    a += b;
    print(&a);
}

#[cfg(not(op_assign))]
fn helper(dims: Dim4) {
    let b = randu::<f32>(dims);
    print(&b);
}

fn handle_backend_test(name: String, backend: Backend) {
    println!("Testing {} backend", name);
    set_backend(backend);
    println!("{0} {1} devices", af::device_count(), name);
    info();
    println!("Creating 10x10 matrix of random floats.");
    helper(Dim4::new(&[10, 10, 1, 1]));
}

fn main() {
    let av = get_available_backends();
    println!("Backend count: {}", get_backend_count());
    println!("Available backends: {:?}", av);

    if av.contains(&Backend::CUDA) {
        handle_backend_test("CUDA".to_string(), Backend::CUDA);
    }

    if av.contains(&Backend::CPU) {
        handle_backend_test("CPU".to_string(), Backend::CPU);
    }

    if av.contains(&Backend::OPENCL) {
        handle_backend_test("OpenCL".to_string(), Backend::OPENCL);
    }
}
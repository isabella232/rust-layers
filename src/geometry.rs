// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Units for use with euclid::length and euclid::scale_factor.

/// One hardware pixel.
///
/// This unit corresponds to the smallest addressable element of the display hardware.
#[derive(Copy, Clone, RustcEncodable, Debug)]
pub enum DevicePixel {}

#[cfg(feature = "heapsize")]
known_heap_size!(0, DevicePixel);

/// One pixel in layer coordinate space.
///
/// This unit corresponds to a "pixel" in layer coordinate space, which after scaling and
/// transformation becomes a device pixel.
#[derive(Copy, Clone, RustcEncodable, Debug)]
pub enum LayerPixel {}

#[cfg(feature = "heapsize")]
known_heap_size!(0, LayerPixel);

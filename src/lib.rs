/*
Copyright 2017 the arrayidx authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::fmt::{Debug};
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};

pub type Index0d = ();
pub type Index1d = usize;
pub type Index2d = [usize; 2];
pub type Index3d = [usize; 3];
pub type Index4d = [usize; 4];
pub type Index5d = [usize; 5];
pub struct UnimplIndex;

pub type Range0d = ();
pub type Range1d = Range<usize>;
pub type Range2d = [Range<usize>; 2];
pub type Range3d = [Range<usize>; 3];
pub type Range4d = [Range<usize>; 4];
pub type Range5d = [Range<usize>; 5];

pub trait ArrayIndex: Clone + PartialEq + Eq + Debug {
  type Range;
  type Above: Sized;

  fn zero() -> Self where Self: Sized;
  fn add(&self, shift: &Self) -> Self where Self: Sized;
  fn sub(&self, shift: &Self) -> Self where Self: Sized;

  fn prepend(&self, new_inside: usize) -> Self::Above;
  fn append(&self, new_outside: usize) -> Self::Above;

  fn to_packed_stride(&self) -> Self where Self: Sized;
  fn is_packed(&self, stride: &Self) -> bool where Self: Sized;
  fn stride_append_packed(&self, outside: usize) -> Self::Above where Self: Sized {
    self.append(self.outside() * outside)
  }

  fn flat_len(&self) -> usize;
  fn flat_index(&self, stride: &Self) -> usize;

  fn inside(&self) -> usize;
  fn outside(&self) -> usize;

  fn dim(&self) -> usize;
}

pub trait ArrayRange<Idx> {
  fn start(&self, offset: &Idx) -> Idx;
  fn end(&self, limit: &Idx) -> Idx;
}

impl ArrayIndex for Index0d {
  type Range = Range0d;
  type Above = Index1d;

  fn zero() -> Self {
    ()
  }

  fn add(&self, shift: &Self) -> Self {
    ()
  }

  fn sub(&self, shift: &Self) -> Self {
    ()
  }

  fn to_packed_stride(&self) -> Self {
    ()
  }

  fn is_packed(&self, stride: &Self) -> bool {
    true
  }

  fn prepend(&self, major: usize) -> Index1d {
    major
  }

  fn append(&self, minor: usize) -> Index1d {
    minor
  }

  fn flat_len(&self) -> usize {
    1
  }

  fn flat_index(&self, stride: &Self) -> usize {
    0
  }

  fn inside(&self) -> usize {
    1
  }

  fn outside(&self) -> usize {
    1
  }

  fn dim(&self) -> usize {
    0
  }
}

impl ArrayIndex for Index1d {
  type Range = Range1d;
  type Above = Index2d;

  fn zero() -> Self {
    1
  }

  fn add(&self, shift: &Self) -> Self {
    *self + *shift
  }

  fn sub(&self, shift: &Self) -> Self {
    *self - *shift
  }

  fn to_packed_stride(&self) -> Self {
    1
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn prepend(&self, major: usize) -> Index2d {
    [major, *self]
  }

  fn append(&self, minor: usize) -> Index2d {
    [*self, minor]
  }

  fn flat_len(&self) -> usize {
    *self
  }

  fn flat_index(&self, stride: &Self) -> usize {
    (*self * *stride) as _
  }

  fn inside(&self) -> usize {
    *self
  }

  fn outside(&self) -> usize {
    *self
  }

  fn dim(&self) -> usize {
    1
  }
}

impl ArrayIndex for Index2d {
  type Range = Range2d;
  type Above = Index3d;

  fn zero() -> Self {
    [0, 0]
  }

  fn add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1], ]
  }

  fn sub(&self, shift: &Self) -> Self {
    [ self[0] - shift[0],
      self[1] - shift[1], ]
  }

  fn to_packed_stride(&self) -> Self {
    let mut s = [0, 0];
    s[0] = 1;
    s[1] = s[0] * self[0];
    s
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn prepend(&self, major: usize) -> Index3d {
    [major, self[0], self[1]]
  }

  fn append(&self, minor: usize) -> Index3d {
    [self[0], self[1], minor]
  }

  fn flat_len(&self) -> usize {
    self[0] * self[1]
  }

  fn flat_index(&self, stride: &Self) -> usize {
    ( self[0] * stride[0] +
      self[1] * stride[1] ) as _
  }

  fn inside(&self) -> usize {
    self[0]
  }

  fn outside(&self) -> usize {
    self[1]
  }

  fn dim(&self) -> usize {
    2
  }
}

impl ArrayIndex for Index3d {
  type Range = Range3d;
  type Above = Index4d;

  fn zero() -> Self {
    [0, 0, 0]
  }

  fn add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2], ]
  }

  fn sub(&self, shift: &Self) -> Self {
    [ self[0] - shift[0],
      self[1] - shift[1],
      self[2] - shift[2], ]
  }

  fn to_packed_stride(&self) -> Self {
    let mut s = [0, 0, 0];
    s[0] = 1;
    s[1] = s[0] * self[0];
    s[2] = s[1] * self[1];
    s
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn prepend(&self, major: usize) -> Index4d {
    [major, self[0], self[1], self[2]]
  }

  fn append(&self, minor: usize) -> Index4d {
    [self[0], self[1], self[2], minor]
  }

  fn flat_len(&self) -> usize {
    self[0] * self[1] * self[2]
  }

  fn flat_index(&self, stride: &Self) -> usize {
    ( self[0] * stride[0] +
      self[1] * stride[1] +
      self[2] * stride[2] ) as _
  }

  fn inside(&self) -> usize {
    self[0]
  }

  fn outside(&self) -> usize {
    self[2]
  }

  fn dim(&self) -> usize {
    3
  }
}

impl ArrayIndex for Index4d {
  type Range = Range4d;
  type Above = Index5d;

  fn add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2],
      self[3] + shift[3], ]
  }

  fn sub(&self, shift: &Self) -> Self {
    [ self[0] - shift[0],
      self[1] - shift[1],
      self[2] - shift[2],
      self[3] - shift[3], ]
  }

  fn to_packed_stride(&self) -> Self {
    let mut s = [0, 0, 0, 0];
    s[0] = 1;
    s[1] = s[0] * self[0];
    s[2] = s[1] * self[1];
    s[3] = s[2] * self[2];
    s
  }

  fn zero() -> Self {
    [0, 0, 0, 0]
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn prepend(&self, major: usize) -> Index5d {
    [major, self[0], self[1], self[2], self[3]]
  }

  fn append(&self, minor: usize) -> Index5d {
    [self[0], self[1], self[2], self[3], minor]
  }

  fn flat_len(&self) -> usize {
    self[0] * self[1] * self[2] * self[3]
  }

  fn flat_index(&self, stride: &Self) -> usize {
    ( self[0] * stride[0] +
      self[1] * stride[1] +
      self[2] * stride[2] +
      self[3] * stride[3] ) as _
  }

  fn inside(&self) -> usize {
    self[0]
  }

  fn outside(&self) -> usize {
    self[3]
  }

  fn dim(&self) -> usize {
    4
  }
}

impl ArrayIndex for Index5d {
  type Range = Range5d;
  type Above = UnimplIndex;

  fn zero() -> Self {
    [0, 0, 0, 0, 0]
  }

  fn add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2],
      self[3] + shift[3],
      self[4] + shift[4], ]
  }

  fn sub(&self, shift: &Self) -> Self {
    [ self[0] - shift[0],
      self[1] - shift[1],
      self[2] - shift[2],
      self[3] - shift[3],
      self[4] - shift[4], ]
  }

  fn to_packed_stride(&self) -> Self {
    let mut s = [0, 0, 0, 0, 0];
    s[0] = 1;
    s[1] = s[0] * self[0];
    s[2] = s[1] * self[1];
    s[3] = s[2] * self[2];
    s[4] = s[3] * self[3];
    s
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn prepend(&self, major: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn append(&self, minor: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn flat_len(&self) -> usize {
    self[0] * self[1] * self[2] * self[3] * self[4]
  }

  fn flat_index(&self, stride: &Self) -> usize {
    ( self[0] * stride[0] +
      self[1] * stride[1] +
      self[2] * stride[2] +
      self[3] * stride[3] +
      self[4] * stride[4] ) as _
  }

  fn inside(&self) -> usize {
    self[0]
  }

  fn outside(&self) -> usize {
    self[4]
  }

  fn dim(&self) -> usize {
    5
  }
}

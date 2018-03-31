/*
Copyright 2017-2018 Peter Jin

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

#![feature(collections_range)]

use std::collections::{Bound};
use std::collections::range::{RangeArgument};
use std::fmt::{Debug};
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};

pub type Index0d = ();
pub type Index1d = usize;
pub type Index2d = [usize; 2];
pub type Index3d = [usize; 3];
pub type Index4d = [usize; 4];
pub type Index5d = [usize; 5];
pub struct UnimplIndex;

pub trait ArrayIndex: Clone + PartialEq + Eq + Debug {
  type Above: Sized;

  fn zero() -> Self where Self: Sized;
  fn index_add(&self, shift: &Self) -> Self where Self: Sized;
  fn index_sub(&self, shift: &Self) -> Self where Self: Sized;

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

impl ArrayIndex for Index0d {
  type Above = Index1d;

  fn zero() -> Self {
    ()
  }

  fn index_add(&self, shift: &Self) -> Self {
    ()
  }

  fn index_sub(&self, shift: &Self) -> Self {
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
  type Above = Index2d;

  fn zero() -> Self {
    0
  }

  fn index_add(&self, shift: &Self) -> Self {
    *self + *shift
  }

  fn index_sub(&self, shift: &Self) -> Self {
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
  type Above = Index3d;

  fn zero() -> Self {
    [0, 0]
  }

  fn index_add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1], ]
  }

  fn index_sub(&self, shift: &Self) -> Self {
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
  type Above = Index4d;

  fn zero() -> Self {
    [0, 0, 0]
  }

  fn index_add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2], ]
  }

  fn index_sub(&self, shift: &Self) -> Self {
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
  type Above = Index5d;

  fn index_add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2],
      self[3] + shift[3], ]
  }

  fn index_sub(&self, shift: &Self) -> Self {
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
  type Above = UnimplIndex;

  fn zero() -> Self {
    [0, 0, 0, 0, 0]
  }

  fn index_add(&self, shift: &Self) -> Self {
    [ self[0] + shift[0],
      self[1] + shift[1],
      self[2] + shift[2],
      self[3] + shift[3],
      self[4] + shift[4], ]
  }

  fn index_sub(&self, shift: &Self) -> Self {
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

pub fn range2idxs_1d<R>(r: R, size: usize) -> (usize, usize)
where R: RangeArgument<usize>,
{
  let start_idx = match r.start() {
    Bound::Included(&x) => x,
    Bound::Excluded(&x) => x + 1,
    Bound::Unbounded => 0,
  };
  let end_idx = match r.end() {
    Bound::Included(&x) => x + 1,
    Bound::Excluded(&x) => x,
    Bound::Unbounded => size,
    _ => unimplemented!(),
  };
  assert!(start_idx <= end_idx);
  assert!(end_idx <= size);
  (start_idx, end_idx)
}

pub fn range2idxs_2d<R0, R1>(r0: R0, r1: R1, size: [usize; 2]) -> ([usize; 2], [usize; 2])
where R0: RangeArgument<usize>,
      R1: RangeArgument<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let start_idx = [s0, s1];
  let end_idx = [e0, e1];
  (start_idx, end_idx)
}

pub fn range2idxs_3d<R0, R1, R2>(r0: R0, r1: R1, r2: R2, size: [usize; 3]) -> ([usize; 3], [usize; 3])
where R0: RangeArgument<usize>,
      R1: RangeArgument<usize>,
      R2: RangeArgument<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let (s2, e2) = range2idxs_1d(r2, size[2]);
  let start_idx = [s0, s1, s2];
  let end_idx = [e0, e1, e2];
  (start_idx, end_idx)
}

pub fn range2idxs_4d<R0, R1, R2, R3>(r0: R0, r1: R1, r2: R2, r3: R3, size: [usize; 4]) -> ([usize; 4], [usize; 4])
where R0: RangeArgument<usize>,
      R1: RangeArgument<usize>,
      R2: RangeArgument<usize>,
      R3: RangeArgument<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let (s2, e2) = range2idxs_1d(r2, size[2]);
  let (s3, e3) = range2idxs_1d(r3, size[3]);
  let start_idx = [s0, s1, s2, s3];
  let end_idx = [e0, e1, e2, e3];
  (start_idx, end_idx)
}

use std::collections::{Bound};
use std::fmt::{Debug};
use std::hash::{Hash};
use std::ops::{Index, RangeBounds};

// TODO: figure out axis API.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ax(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UnimplIndex;

pub type Index0d = ();
pub type Index1d = usize;
pub type Index2d = [usize; 2];
pub type Index3d = [usize; 3];
pub type Index4d = [usize; 4];
pub type Index5d = [usize; 5];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct IndexNd{pub components: Vec<usize>}

impl Default for IndexNd {
  fn default() -> Self {
    IndexNd{components: vec![]}
  }
}

impl Index<usize> for IndexNd {
  type Output = usize;

  fn index(&self, index: usize) -> &usize {
    &self.components[index]
  }
}

impl IndexNd {
  pub fn from(components: Vec<usize>) -> Self {
    IndexNd{components}
  }

  pub fn zero(dim: usize) -> Self {
    let mut components = Vec::with_capacity(dim);
    for _ in 0 .. dim {
      components.push(0);
    }
    IndexNd{components}
  }

  pub fn flat_len(&self) -> usize {
    let mut len = 1;
    for d in 0 .. self.dim() {
      len *= self.components[d];
    }
    len
  }

  pub fn index_at(&self, axis: isize) -> usize {
    self.components[axis as usize]
  }

  pub fn to_packed_stride(&self) -> Self {
    let mut stride = IndexNd::zero(self.dim());
    for d in 0 .. self.dim() {
      match d {
        0 => stride.components[0] = 1,
        _ => stride.components[d] = stride.components[d-1] * self.components[d-1],
      }
    }
    stride
  }

  pub fn is_zero(&self) -> bool {
    for d in 0 .. self.dim() {
      if self.components[d] != 0 {
        return false;
      }
    }
    true
  }

  pub fn is_packed(&self, stride: &Self) -> bool {
    &self.to_packed_stride() == stride
  }

  pub fn inside(&self) -> usize {
    self.components[0]
  }

  pub fn outside(&self) -> usize {
    self.components[self.dim() - 1]
  }

  pub fn dim(&self) -> usize {
    self.components.len()
  }

  pub fn ndim(&self) -> usize {
    self.dim()
  }

  pub fn splice_at(&self, axis: isize) -> (IndexNd, IndexNd, IndexNd) {
    let mut prefix_idx = IndexNd::default();
    for prefix_axis in 0 .. axis {
      prefix_idx.components.push(self.index_at(prefix_axis));
    }
    let mut select_idx = IndexNd::default();
    if axis < self.dim() as isize {
      select_idx.components.push(self.index_at(axis));
    }
    let mut suffix_idx = IndexNd::default();
    for suffix_axis in axis + 1 .. self.dim() as isize {
      suffix_idx.components.push(self.index_at(suffix_axis));
    }
    (prefix_idx, select_idx, suffix_idx)
  }
}

pub trait ArrayIndex: Clone + PartialEq + Eq + Hash + Debug {
  type Above: ArrayIndex + Sized;
  type Below: ArrayIndex + Sized;

  fn zero() -> Self where Self: Sized;

  fn from_nd(nd_shape: Vec<usize>) -> Self where Self: Sized;
  fn to_nd(&self) -> Vec<usize>;
  fn _to_nd(&self) -> IndexNd {
    IndexNd{components: self.to_nd()}
  }

  fn index_add(&self, shift: &Self) -> Self where Self: Sized;
  fn index_sub(&self, shift: &Self) -> Self where Self: Sized;

  fn index_prepend(&self, new_inside: usize) -> Self::Above;
  fn index_append(&self, new_outside: usize) -> Self::Above;

  fn index_at(&self, axis: isize) -> usize;
  fn index_cut(&self, axis: isize) -> Self::Below;

  fn to_packed_stride(&self) -> Self where Self: Sized;
  fn is_packed(&self, stride: &Self) -> bool where Self: Sized;
  fn stride_append_packed(&self, outside: usize) -> Self::Above where Self: Sized {
    self.index_append(self.outside() * outside)
  }

  fn flat_len(&self) -> usize;
  fn flat_index(&self, stride: &Self) -> usize;

  fn inside(&self) -> usize;
  fn outside(&self) -> usize;

  fn dim(&self) -> usize;
  fn ndim(&self) -> usize {
    self.dim()
  }
}

impl ArrayIndex for Index0d {
  type Above = Index1d;
  type Below = Index0d;

  fn zero() -> Self {
    ()
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(0, nd_shape.len());
    ()
  }

  fn to_nd(&self) -> Vec<usize> {
    vec![]
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

  fn index_prepend(&self, major: usize) -> Index1d {
    major
  }

  fn index_append(&self, minor: usize) -> Index1d {
    minor
  }

  fn index_at(&self, _axis: isize) -> usize {
    unreachable!();
  }

  fn index_cut(&self, _axis: isize) -> Index0d {
    // TODO: any special handling for this case?
    ()
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
  type Below = Index0d;

  fn zero() -> Self {
    0
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(1, nd_shape.len());
    nd_shape[0]
  }

  fn to_nd(&self) -> Vec<usize> {
    vec![*self]
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

  fn index_prepend(&self, major: usize) -> Index2d {
    [major, *self]
  }

  fn index_append(&self, minor: usize) -> Index2d {
    [*self, minor]
  }

  fn index_at(&self, axis: isize) -> usize {
    assert_eq!(0, axis);
    *self
  }

  fn index_cut(&self, axis: isize) -> Index0d {
    assert_eq!(0, axis);
    ()
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
  type Below = Index1d;

  fn zero() -> Self {
    [0, 0]
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(2, nd_shape.len());
    [ nd_shape[0],
      nd_shape[1], ]
  }

  fn to_nd(&self) -> Vec<usize> {
    (self as &[usize]).to_owned()
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

  fn index_prepend(&self, major: usize) -> Index3d {
    [major, self[0], self[1]]
  }

  fn index_append(&self, minor: usize) -> Index3d {
    [self[0], self[1], minor]
  }

  fn index_at(&self, axis: isize) -> usize {
    self[axis as usize]
  }

  fn index_cut(&self, axis: isize) -> Index1d {
    match axis {
      0 => self[1],
      1 => self[0],
      _ => unreachable!(),
    }
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
  type Below = Index2d;

  fn zero() -> Self {
    [0, 0, 0]
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(3, nd_shape.len());
    [ nd_shape[0],
      nd_shape[1],
      nd_shape[2], ]
  }

  fn to_nd(&self) -> Vec<usize> {
    (self as &[usize]).to_owned()
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

  fn index_prepend(&self, major: usize) -> Index4d {
    [major, self[0], self[1], self[2]]
  }

  fn index_append(&self, minor: usize) -> Index4d {
    [self[0], self[1], self[2], minor]
  }

  fn index_at(&self, axis: isize) -> usize {
    self[axis as usize]
  }

  fn index_cut(&self, axis: isize) -> Index2d {
    match axis {
      0 => [self[1], self[2]],
      1 => [self[0], self[2]],
      2 => [self[0], self[1]],
      _ => unreachable!(),
    }
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
  type Below = Index3d;

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

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(4, nd_shape.len());
    [ nd_shape[0],
      nd_shape[1],
      nd_shape[2],
      nd_shape[3], ]
  }

  fn to_nd(&self) -> Vec<usize> {
    (self as &[usize]).to_owned()
  }

  fn is_packed(&self, stride: &Self) -> bool {
    self.to_packed_stride() == *stride
  }

  fn index_prepend(&self, major: usize) -> Index5d {
    [major, self[0], self[1], self[2], self[3]]
  }

  fn index_append(&self, minor: usize) -> Index5d {
    [self[0], self[1], self[2], self[3], minor]
  }

  fn index_at(&self, axis: isize) -> usize {
    self[axis as usize]
  }

  fn index_cut(&self, axis: isize) -> Index3d {
    match axis {
      0 => [self[1], self[2], self[3]],
      1 => [self[0], self[2], self[3]],
      2 => [self[0], self[1], self[3]],
      3 => [self[0], self[1], self[2]],
      _ => unreachable!(),
    }
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
  type Below = Index4d;

  fn zero() -> Self {
    [0, 0, 0, 0, 0]
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    assert_eq!(5, nd_shape.len());
    [ nd_shape[0],
      nd_shape[1],
      nd_shape[2],
      nd_shape[3],
      nd_shape[4], ]
  }

  fn to_nd(&self) -> Vec<usize> {
    (self as &[usize]).to_owned()
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

  fn index_prepend(&self, major: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn index_append(&self, minor: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn index_at(&self, axis: isize) -> usize {
    self[axis as usize]
  }

  fn index_cut(&self, axis: isize) -> Index4d {
    match axis {
      0 => [self[1], self[2], self[3], self[4]],
      1 => [self[0], self[2], self[3], self[4]],
      2 => [self[0], self[1], self[3], self[4]],
      3 => [self[0], self[1], self[2], self[4]],
      4 => [self[0], self[1], self[2], self[3]],
      _ => unreachable!(),
    }
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

impl ArrayIndex for UnimplIndex {
  type Above = UnimplIndex;
  type Below = UnimplIndex;

  fn zero() -> Self {
    unimplemented!();
  }

  fn from_nd(nd_shape: Vec<usize>) -> Self {
    unimplemented!();
  }

  fn to_nd(&self) -> Vec<usize> {
    unimplemented!();
  }

  fn index_add(&self, shift: &Self) -> Self {
    unimplemented!();
  }

  fn index_sub(&self, shift: &Self) -> Self {
    unimplemented!();
  }

  fn to_packed_stride(&self) -> Self {
    unimplemented!();
  }

  fn is_packed(&self, stride: &Self) -> bool {
    unimplemented!();
  }

  fn index_prepend(&self, major: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn index_append(&self, minor: usize) -> UnimplIndex {
    unimplemented!();
  }

  fn index_at(&self, axis: isize) -> usize {
    unimplemented!();
  }

  fn index_cut(&self, axis: isize) -> UnimplIndex {
    unimplemented!();
  }

  fn flat_len(&self) -> usize {
    unimplemented!();
  }

  fn flat_index(&self, stride: &Self) -> usize {
    unimplemented!();
  }

  fn inside(&self) -> usize {
    unimplemented!();
  }

  fn outside(&self) -> usize {
    unimplemented!();
  }

  fn dim(&self) -> usize {
    unimplemented!();
  }
}

pub fn range2idxs_1d<R>(r: R, size: usize) -> (usize, usize)
where R: RangeBounds<usize>,
{
  let start_idx = match r.start_bound() {
    Bound::Included(&x) => x,
    Bound::Excluded(&x) => x + 1,
    Bound::Unbounded => 0,
  };
  let end_idx = match r.end_bound() {
    Bound::Included(&x) => x + 1,
    Bound::Excluded(&x) => x,
    Bound::Unbounded => size,
  };
  assert!(start_idx <= end_idx,
      "array bounds violation: {} greater than {}", start_idx, end_idx);
  assert!(end_idx <= size,
      "array end bounds violation: {} greater than {}", end_idx, size);
  (start_idx, end_idx)
}

/*pub fn unzip_range_3d<RR>(rr: RR, size: [usize; 3]) -> (Range<usize>, Range<usize>, Range<usize>)
where RR: RangeBounds<[usize; 3]>
{
  // TODO
}*/

pub fn range2idxs_2d<R0, R1>(r0: R0, r1: R1, size: [usize; 2]) -> ([usize; 2], [usize; 2])
where R0: RangeBounds<usize>,
      R1: RangeBounds<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let start_idx = [s0, s1];
  let end_idx = [e0, e1];
  (start_idx, end_idx)
}

pub fn range2idxs_3d<R0, R1, R2>(r0: R0, r1: R1, r2: R2, size: [usize; 3]) -> ([usize; 3], [usize; 3])
where R0: RangeBounds<usize>,
      R1: RangeBounds<usize>,
      R2: RangeBounds<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let (s2, e2) = range2idxs_1d(r2, size[2]);
  let start_idx = [s0, s1, s2];
  let end_idx = [e0, e1, e2];
  (start_idx, end_idx)
}

pub fn range2idxs_4d<R0, R1, R2, R3>(r0: R0, r1: R1, r2: R2, r3: R3, size: [usize; 4]) -> ([usize; 4], [usize; 4])
where R0: RangeBounds<usize>,
      R1: RangeBounds<usize>,
      R2: RangeBounds<usize>,
      R3: RangeBounds<usize>,
{
  let (s0, e0) = range2idxs_1d(r0, size[0]);
  let (s1, e1) = range2idxs_1d(r1, size[1]);
  let (s2, e2) = range2idxs_1d(r2, size[2]);
  let (s3, e3) = range2idxs_1d(r3, size[3]);
  let start_idx = [s0, s1, s2, s3];
  let end_idx = [e0, e1, e2, e3];
  (start_idx, end_idx)
}

pub trait AsIndex {
	fn to_idx(&self) -> usize;
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct VariantIndex<E: AsIndex, O, const N: usize> {
	pub array: [O; N],
	_marker: std::marker::PhantomData<E>,
}

impl<E: AsIndex, O, const N: usize> VariantIndex<E, O, N> {
	pub const fn new(array: [O; N]) -> Self {
		VariantIndex { array, _marker: std::marker::PhantomData }
	}
}

impl<E: AsIndex, O, const N: usize> std::ops::Index<E> for VariantIndex<E, O, N> {
	type Output = O;

	fn index(&self, variant: E) -> &Self::Output {
		&self.array[variant.to_idx()]
	}
}

impl<E: AsIndex, O, const N: usize> std::ops::IndexMut<E> for VariantIndex<E, O, N> {
	fn index_mut(&mut self, variant: E) -> &mut Self::Output {
		&mut self.array[variant.to_idx()]
	}
}

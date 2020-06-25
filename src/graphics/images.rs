use std::marker::PhantomData;

pub struct Image<P> {
  _format: PhantomData<P>,
}
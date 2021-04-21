#![allow(dead_code)]
#![allow(unused_imports)]
pub mod vecmath;
pub use vecmath::*;
use std::{cmp,ops};
use ops::{Mul,Add,Index,AddAssign};
use std::fmt::Debug;

// pub mod whatever would go here for more src

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub trait MyIndex :Copy+Default+Ord{ fn to_usize(self)->usize; fn from_usize(u:usize)->Self; }
impl MyIndex for u32 { fn to_usize(self)->usize{self as usize}  fn from_usize(u:usize)->Self {u as u32}}
impl MyIndex for usize { fn to_usize(self)->usize{self as usize}  fn from_usize(u:usize)->Self {u as usize}}

#[derive(Debug,Default)]
pub struct SparseMatrixCOO<T,Index=u32> {
	pub rows_columns:(Index,Index),
	pub values:Vec<(T,(Index,Index))>,	// edge data
}

impl<T,Index:MyIndex> SparseMatrixCOO<T,Index> {
//	pub fn new()->Self{SparseMatrixCOO{rows_columns:(Index::default(),Index::default()),values:vec![]}}
	pub fn push(&mut self, (v,(r,c)):(T,(Index,Index))){
		// when inserting elements just assume the whole size must contain them.
		self.rows_columns=(cmp::max(r,self.rows_columns.0),cmp::max(c,self.rows_columns.1));
		self.values.push((v,(r,c)));
	}
}
pub trait SparseMatrix<A,I:MyIndex>
{
	fn foreach_mut<F:Fn(&mut A,(I,I))>(&mut self,f:F);
	fn mul_dense_vec<B,C>(&self,src:&Vec<B>)->Vec<C> where
		for <'x,'y> &'x A:Mul<&'y B,Output=C>,
		C:'static+Default+AddAssign+Clone;
}

impl<A,I:MyIndex> SparseMatrix<A,I> for SparseMatrixCOO<A,I> 
{

	fn mul_dense_vec<B,C>(&self,src:&Vec<B>)->Vec<C> where
		for <'x,'y> &'x A:Mul<&'y B,Output=C>,
		C:'static+Default+AddAssign+Clone
			
	{
		let mut res=Vec::new();
		// todo actually this just assumed it's a square matrix, which it needn't be.
		res.resize(src.len(), C::default());
		for (ref val,(row,col)) in self.values.iter() {
			res[row.to_usize()]+=val*&src[col.to_usize()]
		}
		res
	}

	fn foreach_mut<F:Fn(&mut A,(I,I))>(&mut self,f:F) {
		for &mut (ref mut v,rc) in self.values.iter_mut() {
			f(v,rc)
		}
	}

}


impl<'a,'b, A,B,C,I:MyIndex>  Mul<&'b Vec<B>> for    &'a SparseMatrixCOO<A,I>
   where
	for<'x> &'x B:Add,
        C:'static+AddAssign+Default+Clone,
        for<'x,'y> &'x A: Mul<&'y B, Output=C>,

{
	type Output = Vec<C> ;
	fn mul(self,src:&Vec<B>)->Self::Output {
		let mut res=Vec::new();
		// todo actually this just assumed it's a square matrix, which it needn't be.
		res.resize(src.len(), C::default());
		for (ref val,(row,col)) in self.values.iter() {
			res[row.to_usize()]+=val*&src[col.to_usize()]
		}
		res
	}
}


/// a 'Graph' pairs a node array with a sparse matrix of edge connections.
#[derive(Debug)]
pub struct Graph<N,E,Index=u32>{
	pub nodes:Vec<N>,
	pub edges:SparseMatrixCOO<E,Index>
}


impl<E,N,Prod, I:MyIndex> Graph<N,E,I> 
    where
	E:Debug+Default, 
	N:Debug+Default, 
        for<'x,'y> &'x E:Mul<&'y N,Output=Prod>,
        Prod:'static+AddAssign+Default+Clone
    {
	
	pub fn new()->Self{
		Graph{nodes:vec![],edges:SparseMatrixCOO::default()}
	}
	pub fn add_node(&mut self,n:N)->I{
		self.nodes.push(n);
		MyIndex::from_usize(self.nodes.len()-1)		
	}
	pub fn add_edge(&mut self,e:E,vs:I,ve:I){
		// in matrix terms, 'output' = row index, 'input'=column index.
		self.edges.push((e,(ve,vs)))

	}


	// apply a function to modify every edge
	pub fn update_edges<F:Fn(&mut E,&N,&N)>(&mut self,f:F){
		Self::update_edges_sub(&mut self.nodes,&mut self.edges,f);
	}
	// subroutine satisies the borrow checker that we're not mutating 'self' in two places.
	fn update_edges_sub<F:Fn(&mut E,&N,&N)>(nodes:&mut [N],edges:&mut SparseMatrixCOO<E,I>, f:F) {
		edges.foreach_mut(|mut val,rc| {
			f(val, &nodes[rc.1.to_usize()], &nodes[rc.0.to_usize()]);
		});
	}

	pub fn update_along_edges<UpdateF:Fn(&mut N,&Prod)>	// function to update node with accumulated messages 
		(
			&mut self,  
			updater:UpdateF
		)
	{
		let mut acc=vec![Prod::default();self.nodes.len()];
		let acc = self.edges.mul_dense_vec(&self.nodes);
		
		for (i,a) in acc.iter().enumerate(){	//todo - zip iterator
			updater(&mut self.nodes[i],a)
		}
	}

	pub fn foreach_node<F:FnMut(&N)>(&self,mut f:F) {
		for n in self.nodes.iter() { f(n) }
	}
	pub fn foreach_edge<F:FnMut(&E,&N,&N)>(&self,mut f:F) {
		for &(ref e,(ei,si)) in self.edges.values.iter() { 
			f(e,
				&self.nodes[si.to_usize()],
				&self.nodes[ei.to_usize()] ) 
		}
	}

}


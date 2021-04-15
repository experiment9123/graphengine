#![allow(dead_code)]

// pub mod whatever would go here

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



pub trait MyIndex :Copy{ fn to_usize(self)->usize; fn from_usize(u:usize)->Self; }
impl MyIndex for u32 { fn to_usize(self)->usize{self as usize}  fn from_usize(u:usize)->Self {u as u32}}
impl MyIndex for usize { fn to_usize(self)->usize{self as usize}  fn from_usize(u:usize)->Self {u as usize}}



#[derive(Debug)]
pub struct Graph<N,E,Index=u32>{
	nodes:Vec<N>,
	edges:Vec<(E,(Index,Index))>,	// edge data
}

impl<N,E,I:MyIndex> Graph<N,E,I>{
	
	pub fn new()->Self{
		Graph{nodes:vec![],edges:vec![]}
	}
	pub fn add_node(&mut self,n:N)->I{
		self.nodes.push(n);
		MyIndex::from_usize(self.nodes.len()-1)		
	}
	pub fn add_edge(&mut self,e:E,vs:I,ve:I){
		self.edges.push((e,(vs,ve)))

	}

	// apply a function to modify every edge
	pub fn update_edges<F:Fn(&mut E,&N,&N)>(&mut self,f:F){
		for &mut (ref mut e,(si,ei)) in self.edges.iter_mut() {
			f(e, &self.nodes[si.to_usize()],&self.nodes[ei.to_usize()])
		}
	}

	pub fn update_along_edges<
			Message,
			AccMsg:Default+Clone,
			SendF:Fn(&N,&E)->Message,	// function to generate message
			AccMsgF:Fn(&mut AccMsg,Message),	// function to accumulate message (consuming it)
			UpdateF:Fn(&mut N,&AccMsg)>	// function to update node with accumulated messages 
		(
			&mut self,  
			sender:SendF,
			accumulate:AccMsgF,
			updater:UpdateF
		)
	{
		// TODO - optimized threaded version..

		let mut acc=vec![AccMsg::default();self.nodes.len()];
		for &(ref e,(si,ei)) in self.edges.iter() {
			let msg = sender(&self.nodes[si.to_usize()],e);
			accumulate(&mut acc[ei.to_usize()],msg);
		}
		
		for (i,a) in acc.iter().enumerate(){
			updater(&mut self.nodes[i],a)
		}
	}

}


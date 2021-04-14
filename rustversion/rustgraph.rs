#![allow(dead_code)]

type Index=usize;

#[derive(Debug)]
struct Graph<N,E>{
	nodes:Vec<N>,
	edges:Vec<(E,(Index,Index))>,	// edge data
}

impl<N,E> Graph<N,E>{
	
	fn new()->Self{
		Graph{nodes:vec![],edges:vec![]}
	}
	fn add_node(&mut self,n:N)->Index{
		self.nodes.push(n);
		self.nodes.len()-1		
	}
	fn add_edge(&mut self,e:E,vs:Index,ve:Index){
		self.edges.push((e,(vs,ve)))

	}

	// apply a function to modify every edge
	fn update_edges<F:Fn(&mut E,&N,&N)>(&mut self,f:F){
		for &mut (ref mut e,(si,ei)) in self.edges.iter_mut() {
			f(e, &self.nodes[si],&self.nodes[ei])
		}
	}

	fn update_along_edges<
			Message,
			AccMsg:Clone,
			SendF:Fn(&N,&E)->Message,	// function to generate message
			AccMsgF:Fn(&mut AccMsg,Message),	// function to accumulate message (consuming it)
			UpdateF:Fn(&mut N,&AccMsg)>	// function to update node with accumulated messages 
		(
			&mut self,
			a:AccMsg,			// init val of message accumulator
			sender:SendF,
			accumulate:AccMsgF,
			updater:UpdateF
		)
	{
		// TODO - optimized threaded version..

		let mut acc=vec![a;self.nodes.len()];
		for &(ref e,(si,ei)) in self.edges.iter() {
			let msg = sender(&self.nodes[si],e);
			accumulate(&mut acc[ei],msg);
		}
		
		for (i,a) in acc.iter().enumerate(){
			updater(&mut self.nodes[i],a)
		}
	}

}



fn main(){
	let mut gr:Graph<String,String>= Graph::new();
	gr.add_node(String::from("node_foo"));
	gr.add_node(String::from("node_bar"));
	gr.add_edge(String::from("edge_foobar"),0,1);
	println!("graph initial state:\n {:?}\n",gr);
	gr.update_along_edges(
		String::from(""), 				// init accumulator state
		|n,e|{format!("(from {} along {})",n,e)},  	// send message from node along edge
		|a,m|{*a=format!("{} received {}",a,m)},		// accumualte messages received along edge
		|n,acc|{n.push_str(acc)}
	);
	println!("graph after update:\n {:?}\n",gr);
	
}

extern crate rustgraph;


fn main(){
	
	let mut graph:Graph<String,String>= Graph::new(); // test the graph type with Strings for nodes & edges
	let n0=graph.add_node(String::from("node_foo"));
	let n1=graph.add_node(String::from("node_bar"));
	graph.add_edge(String::from("edge_foobar"),n0,n1);
	println!("graph initial state:\n {:?}\n",graph);

	// the "science" would go here..
	graph.update_along_edges(
		|_node,_edge|{1u32},  	// send message from node along edge
		|allmsg,msg|{*allmsg+=msg},// accumualte messages received at a node
		|node,allmsg:&u32|{node.push_str(&format!("has {} incoming edges",allmsg))}		// update the node once  all messages are accumulated.
	);
	println!("graph after update:\n {:?}\n",graph);
	graph.foreach_edge(|e,n0,n1|{
		println!("edge:{} start{} end{}\n", e,n0,n1);
	});
}



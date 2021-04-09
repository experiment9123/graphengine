#pragma once
#include <cstdint>
#include <vector>

/*
template<typename NODE,typename EDGE, typename MESSAGE>
concept GraphNode = requires(NODE& node,const EDGE& e,const MESSAGE& m) {
	// expressions that must be possible to write, -> "return type"
	{node.generate_messge(e)}->MESSAGE;
	// todo - 'reduce' collect the messages 
	//and make 'update' take an accumulation of all incoming messages
	{node.receive_message(m)}->MESSAGE;
	{node.update();}->bool;
};

template<typename NODE,typename EDGE, typename MESSAGE>
concept GraphEdge = requires(const NODE& n0,const NODE& n1,const EDGE& e,const MESSAGE& m) {
	{edge.update(n0,n1)}->void;
	{node.receive_message(m)}->MESSAGE;
	{node.update();}->bool;
};
*/
template<typename NODE,typename EDGE=typename NODE::Edge_t,typename MESSAGE=typename NODE::Message_t,typename INDEX=uint32_t>
class GraphEngine {
public:
	struct GraphEdge {
		EDGE data;
		INDEX start,end;
	};
	std::vector<NODE>	m_nodes;
	std::vector<GraphEdge>	m_edges;
	// todo - sort them.. store ranges in edge list instead.
	std::vector<std::vector<INDEX>>	m_edgesPerNode;
	INDEX	create_node(const NODE& n) {
		m_nodes.push_back(n);
		m_edgesPerNode.resize(m_nodes.size());
		return m_nodes.size()-1;
	}

	void create_edge(const EDGE& e,INDEX start,INDEX end){
		auto id=m_edges.size();

		m_edges.push_back(GraphEdge{e,start,end});
		m_edgesPerNode[start].push_back(id);
	}
	void update() {
		// TODO - active list management, and parallel!
		// pass 1: send the message from each node
		for (size_t i=0; i<m_nodes.size(); i++) {
			auto& node=m_nodes[i];
			for (auto ei:m_edgesPerNode[i]) {
				auto& edge=m_edges[ei];
				auto& other=m_nodes[edge.end];
				auto msg=node.generate_message(edge.data);

				other.receive_message(msg);
			}
		}
		// pass 2: update the edges, nodes stay constant. (learning rules can affect edges..)
		for (auto& e:m_edges){
			e.data.update(m_nodes[e.start],m_nodes[e.end]);
		}
		// pass 3: update each node, now that it's sent and received all it's messages.
		for (size_t i=0; i<m_nodes.size(); i++) {
			m_nodes[i].update();	
		}
	}
};



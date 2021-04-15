#pragma once
#include <cstdint>
#include <vector>
#include <algorithm>
#include <functional>

#define GE_ASSERT(x) if (!(x)){printf("%s:%d error %s",__FILE__,__LINE__,#x);exit(0);}

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


template<typename NODE,
	typename EDGE=float,
	typename MESSAGE=decltype(NODE().generate_message(EDGE())),
	typename INDEX=uint32_t>
class GraphEngine {
	struct GraphEdge {
		EDGE data;
		INDEX start,end;
	};
	struct Node {
		NODE data;
		INDEX numEdges;	// describes a range in the edges array.
		INDEX firstEdgeIndex;
	};
	std::vector<Node>	m_nodes;
	std::vector<GraphEdge>	m_edges;
	// todo - sort them.. store ranges in edge list instead.
	bool m_building=false;

public:
	void begin_building(){m_building=true;}
	INDEX	create_node(const NODE& n) {
		GE_ASSERT(m_building && "wrap construction in begin_building/end_building")
		m_nodes.push_back(Node{n,0,0});
		return m_nodes.size()-1;
	}

	void create_edge(const EDGE& e,INDEX start,INDEX end){
		GE_ASSERT(m_building)
	
		auto id=m_edges.size();
		m_edges.push_back(GraphEdge{e,start,end});
	}
	void end_building() {
		GE_ASSERT(m_building)
		m_building=false;
		// todo sort indices for locality...

		std::sort(m_edges.begin(),m_edges.end(), [](auto& a,auto& b){return a.start<b.start;});
		for (auto& n:m_nodes){n.numEdges=0;}
		INDEX lastNodeId=-1;
		for (INDEX ei=0; ei<m_edges.size(); ei++){ 
			auto& e=m_edges[ei];
			m_nodes[e.start].numEdges++; 
			if (e.start!=lastNodeId){lastNodeId=e.start; m_nodes[e.start].firstEdgeIndex=ei;}
		}
	}
	void update() {
		GE_ASSERT(!m_building)
		// pass 1: send the message from each node
		// TODO - active list management, and parallel!
		// - this should only be called for a current 'active list'
		for (size_t i=0; i<m_nodes.size(); i++) {
			auto& node=m_nodes[i];
			if (!node.data.is_active()) continue;	// skip this if not active.
			auto edge=&m_edges[node.firstEdgeIndex];
			for (size_t j=0; j<node.numEdges; j++,edge++) {
				auto& other=m_nodes[edge->end];
				auto msg=node.data.generate_message(edge->data);

				other.data.receive_message(msg);
				// todo - everything that receives a message should be added to the active list.
			}
		}
		// pass 2: update the edges, nodes stay constant. (learning rules can affect edges..)
		for (auto& e:m_edges){
			e.data.update(m_nodes[e.start].data,m_nodes[e.end].data);
		}
		// pass 3: update each node, now that it's sent and received all it's messages.
		for (size_t i=0; i<m_nodes.size(); i++) {
			m_nodes[i].data.update();
		}
	}
	// internal iterators
	void for_each_edge(std::function<void(EDGE& e, NODE& n0,NODE& n1)> f){
		for (auto& e:m_edges){
			f(e.data,m_nodes[e.start].data,m_nodes[e.end].data);
		}
	}
	void for_each_node(std::function<void(NODE& n)> f) { for (auto& n:m_nodes){ f(n.data);} }
	void for_each_edge(std::function<void(const EDGE& e, const NODE& n0,const NODE& n1)> f)const{
		for (auto& e:m_edges){
			f(e.data,m_nodes[e.start].data,m_nodes[e.end].data);
		}
	}
	void for_each_node(std::function<void(const NODE& n)> f) const{ for (auto& n:m_nodes){ f(n.data);} }

};




#pragma once
#include <cstdint>
#include <vector>
#include <algorithm>
#include <functional>

#define GE_ASSERT(x) if (!(x)){printf("%s:%d error %s",__FILE__,__LINE__,#x);exit(0);}
#define GE_TRACE() {printf("%s:%d\n",__FILE__,__LINE__);}


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

/// Sparse Matrix in index list ("COO") format 
/// https://en.wikipedia.org/wiki/Sparse_matrix#Coordinate_list_(COO)
/// compressed formats to come..
template<typename T,typename INDEX=uint32_t>
struct SparseMatrixCOO {
	std::array<INDEX,2> rows_columns={0,0};
	std::vector<Elem> values;
	void reduce_vals();
public:
	struct Elem { T val; INDEX row; INDEX column;};// less error prone to call these 'row,col' explicitely.
	typename T Value;				// easily extracted.
	void insert_at(const T& src, INDEX row,INDEX column){
		modified=true;
		rows_columns={std::max(rows_columns[0],row),std::max(rows_columns[1],column)};
		values.push_back(Elem{src,row,column});
	}
	// iterator yields val,row,col
	auto begin()const {return values.begin();}	
	auto end()const {return values.end();}	
}; 

/// trivial implementation of 'sparse matrix X dense vector'
/// (parallel compressed sparse mat  X SPARSE vector is where it'll get highly non-trivial
/// the trivial impl will verify functionality.
///
/// Output type decltype(A*B+A*B) - "accumulated messages". single message = A*B
template<typename A,typename B,typename INDEX=uint32_t>
std::vector<decltype(A()*B()+A()*B())> operator*(const SparseMatrixCOO<A,INDEX>& mat, const std::vector<B>& srcvec){
	std::vector<decltype(A()*B()+A()*B())> result;
	result.resize(mat.rows_columns[0]);	// output vector, one 'accumulator slot' each.

	for (auto& v : mat.values) {
		result[v.row] += v.val * srcvec[ v.column ];
	}


	return result;
};


/// WIP.. "graph engine" rewritten to hold connections in a SparseMatrix.
/// switches to calling EDGE*NODE,+ to generate and accumulate messages.
/// the 'message' may represent how that value changes in a time-step
template<typename NODE, typename EDGE=float, typename INDEX=uint32_t>
class GraphWithEdgeMatrix {
	std::vector<NODE>	m_nodes;
	SparseMatrixCOO<EDGE>	m_edges;
	typedef typename SparseMatrixCOO<EDGE>::Elem SMElem;
public:
	INDEX	create_node(const NODE& n) {
		m_nodes.push_back(n);
		return m_nodes.size()-1;
	}

	void create_edge(const EDGE& e,INDEX edge_start_index,INDEX edge_end_index){
		// careful! matrix ROW index is output, matrix COLUMN index selects in the input
		m_edges.insert_at(e,edge_end_index,edge_start_index);
	}

	void begin_building() {}
	void end_building() {}
	void update() {
		// uses EDGE*NODE to generate messages, result "+=" to accumulate them
		// then NODE+={result} to update the node.
		auto accmsg = m_edges * m_nodes;
		for (size_t i=0; i<accmsg.size(); i++) {
			m_nodes[i].receive_message(accmsg[i]);
			m_nodes[i].update();
		}
	}
	// mutating version, 'update edges' = learning rule.
	void for_each_edge(std::function<void(EDGE& e, const NODE& n0,const NODE& n1)> f){
		for (auto& ep : m_edges) {
			// TODO double check row/column index are right!
			f( ep.val, m_nodes[ep.column_index],m_nodes[ep.row_index]);
		}
	}
	void for_each_node(std::function<void(NODE& n)> f) { for (auto& n:m_nodes){ f(n);} }
	void for_each_node(std::function<void(const NODE& n)> f) const{ for (auto& n:m_nodes){ f(n);} }

	// non mutating version
	void for_each_edge(std::function<void(const EDGE& e, const NODE& n0,const NODE& n1)> f)const{
		for (auto& ep : m_edges) {
			// TODO double check row/column index are right!
			f( ep.val, m_nodes[ep.column],m_nodes[ep.row]);
		}
	}
};


/// 1st version of 'Graph Engine', holding edge & node structures together with node-edge lists etc.
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





#include "stdio.h"
#include "graphengine.h"
#include <SDL2/SDL.h>
#include <stdio.h>



/*
clang++ gol.cpp -lSDL2 && ./a.out
*/
#define TRACE printf("%s:%s:%d\n",__FILE__,__FUNCTION__,__LINE__);
typedef bool Message;
struct Edge;
struct Cell {
	typedef bool Message_t;
	typedef ::Edge Edge_t;
	int x,y;
	int num_neighbours=0;
	int color=0;
	bool alive=false;
	Message_t generate_message(const Edge_t& e) const{
		// for a neural net, the 'edge' itself
		return this->alive;
	}
	void receive_message(const Message_t& msg) {
		if (msg) {num_neighbours++;}

	}
	void update() {

		if (!alive) {
			if (num_neighbours==3) alive=true;
		} else {
			if (num_neighbours>3 || num_neighbours<2) alive=false;
			
		}


		num_neighbours=0;

	}
	void draw() {
	}
};

struct Edge {
	// no data
	// connection weight would go here
	void update(const Cell& a, const Cell& b) {
		// learning rule adjusting weights would go here
	}
};

void init_grid(GraphEngine<Cell>& gol) {
	const int numx=32;
	const int numy=32;
	const int spacing=16;

	auto gridindex=[](int i,int j){
		if (i<0) i+=numx;
		if (j<0) j+=numy;
		i%=numx;
		j%=numy;
		return i+j*numx;
	
	};

	for (int y=0; y<numy; y++) {
		for (int x=0; x<numx; x++) {
			Cell c;
			c.x=x*spacing;
			c.y=y*spacing;
			auto f= (x&3)+(y&3);
			c.alive=false;//((rand()&7)+3)<f?true:false;
			auto id=gol.create_node(c);

		}
	}
	for (int y=0; y<numy; y++) {
		for (int x=0; x<numx; x++) {
			int index=gridindex(x,y);
			gol.create_edge(Edge{}, index,gridindex(x-1,y));
			gol.create_edge(Edge{}, index,gridindex(x+1,y));
			gol.create_edge(Edge{}, index,gridindex(x,y-1));
			gol.create_edge(Edge{}, index,gridindex(x,y+1));

			gol.create_edge(Edge{}, index,gridindex(x-1,y-1));
			gol.create_edge(Edge{}, index,gridindex(x+1,y-1));
			gol.create_edge(Edge{}, index,gridindex(x-1,y+1));
			gol.create_edge(Edge{}, index,gridindex(x+1,y+1));

		}
	}
	for (int i=0; i<10; i++) {
		auto x=rand()&63;
		auto y=rand()&63;
		gol.m_nodes[gridindex(x,y-1)].alive=true;
		gol.m_nodes[gridindex(x+1,y)].alive=true;
		gol.m_nodes[gridindex(x+1,y+1)].alive=true;
		gol.m_nodes[gridindex(x+0,y+1)].alive=true;
		gol.m_nodes[gridindex(x-1,y+1)].alive=true;
	}
}

void render(SDL_Renderer* rs, GraphEngine<Cell>& gol) {
	for (auto& edge:gol.m_edges) {
		auto& n0=gol.m_nodes[edge.start];
		auto& n1=gol.m_nodes[edge.end];
		SDL_SetRenderDrawColor(rs,128,128,128,255);
		auto dx=(n1.x-n0.x)/2;
		auto dy=(n1.y-n0.y)/2;
		SDL_RenderDrawLine(rs, n0.x,n0.y, n0.x+dx,n0.y+dy);
	}	

	for (auto& node:gol.m_nodes) {
		if (node.alive)
			SDL_SetRenderDrawColor(rs,255,255,255,255);
		else
			SDL_SetRenderDrawColor(rs,32,32,32,255);

		SDL_Rect rc;
		int s=3;
		rc.x=node.x-s;
		rc.y=node.y-s;
		rc.w=s*2;
		rc.h=s*2;
		SDL_RenderFillRect(rs,&rc);
	}
		
}

int main(int argc, const char** argv) {
	SDL_Init(SDL_INIT_VIDEO);
	SDL_Window* win=SDL_CreateWindow("graph engine test",SDL_WINDOWPOS_CENTERED,SDL_WINDOWPOS_CENTERED,1024,768,0);
	SDL_Renderer* rs=SDL_CreateRenderer(win,0,0);
	

	SDL_CreateWindowAndRenderer(1024,768,SDL_WINDOW_OPENGL,&win, &rs);
	GraphEngine<Cell> gol;
	// initialise cells in a grid
	init_grid(gol);
	bool running=true;
	while (running) {
		SDL_Event e;
		while (SDL_PollEvent(&e)) {
			if (e.type == SDL_QUIT) {
				running=false;
			}
			if (e.type == SDL_WINDOWEVENT) {
				if (e.window.event==SDL_WINDOWEVENT_CLOSE){
					running=false;
				}
			}
			if (e.type==SDL_KEYDOWN){
				gol.update();
			
			}
		}
		
		

		SDL_SetRenderDrawColor(rs,0,0,0,255);
		SDL_RenderClear(rs);
		render(rs,gol);
		SDL_RenderPresent(rs);
		SDL_Delay(100);
	}


}



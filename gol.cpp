
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
struct Cell {	// a plugin 'Node' type for the GraphEngine template,implementing GoL
	// typedefs for the associated type are extracted by the template param defaults. you can specifiy them manually aswell to avoid this
	typedef bool Message_t;	// the message type
	typedef ::Edge Edge_t;	// edge edge type connecting nodes.
	int x,y;
	int num_neighbours=0;	// 'message accumulator' held permanently in the cell. (TODO .. seperate accumulator type?)
	bool alive=false;

	// called along each outgoing edge from this node
	Message_t generate_message(const Edge_t& e) const{
		// for a neural net, 'Edge' weighting would be used here
		// for game of life the message is merely "1" or "0"
		return this->alive;
	}
	// called for each incoming Edge to this node
	void receive_message(const Message_t& msg) {
		if (msg) {num_neighbours++;}

	}
	// called once all the messages have been received.
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

void init_grid(GraphEngine<Cell>& gol,int winx,int winy) {
	const int numx=64;
	const int numy=64;
	const int spacing=winx/numx;

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
			c.alive=((rand()&7)+1)<f?true:false;
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
		gol.m_nodes[gridindex(x+0,y-1)].alive=true;
		gol.m_nodes[gridindex(x+1,y)].alive=true;
		gol.m_nodes[gridindex(x+1,y+1)].alive=true;
		gol.m_nodes[gridindex(x+0,y+1)].alive=true;
		gol.m_nodes[gridindex(x-1,y+1)].alive=true;
	}
}

void render(SDL_Renderer* rs, GraphEngine<Cell>& gol) {
	SDL_SetRenderDrawBlendMode(rs,SDL_BLENDMODE_BLEND);
	for (auto& edge:gol.m_edges) {
		auto& n0=gol.m_nodes[edge.start];
		auto& n1=gol.m_nodes[edge.end];
		SDL_SetRenderDrawColor(rs,0,128,255,32);
		auto dx=(n1.x-n0.x);
		auto dy=(n1.y-n0.y);
		
//		if (dx<-32 || dx>32 || dy<-32 || dy>32)continue;// hack , dont draw the wraparound links, they look messy
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
	int width=1024;
	int height=1024;
	

	SDL_CreateWindowAndRenderer(width,height,SDL_WINDOW_OPENGL,&win, &rs);
	GraphEngine<Cell> gol;
	// initialise cells in a grid
	init_grid(gol,width,height);
	bool running=true;
	bool paused=false;
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
				switch (e.key.keysym.sym) {
					case SDLK_SPACE: paused^=1;break;
					case SDLK_RETURN: 
						for (int i=0; i<500; i++){
							gol.m_nodes[rand()%gol.m_nodes.size()].alive=true;
						}
					break;
				}
			}
		}
		
		if (!paused)
			gol.update();
		

		SDL_SetRenderDrawColor(rs,0,0,0,255);
		SDL_RenderClear(rs);
		render(rs,gol);
		SDL_RenderPresent(rs);
		SDL_Delay(50);
	}


}



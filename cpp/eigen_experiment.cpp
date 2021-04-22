#pragma once
#include <Eigen/Sparse>

class Foo {public:Foo(){} Foo(int x){}};
class Bar {public:Bar(){} Bar(int x){}};
class Baz {public:Baz(){} Baz(int x){}};
class Qux {public:Qux(){} Qux(const Baz&){printf("Qux(Baz)");} Qux(Baz&){printf("Qux(Baz)");} Qux(int x){}};
auto operator*(const Foo& a,const Bar& b){printf("Foo*Bar\n");return Baz{};}
auto operator+(const Baz& a,const Baz& b){printf("Baz+Baz\n");return Baz{};}
auto operator+=(Baz& a,const Baz& b){printf("Baz+=Baz\n");return a;}
auto operator+=(Qux& a,const Baz& b){printf("Qux+=Baz\n");return a;}
template<>
class Eigen::NumTraits<Foo> {
public:
	typedef Foo Real;
	typedef Foo NonInteger;	
	typedef Foo Literal;
	typedef Foo Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Foo();}
	auto static dummy_precision(){return Foo();}
	auto static highest(){return Foo();}
	auto static lowest(){return Foo();}
	auto static digist10(){return 5;}
};

template<>
class Eigen::NumTraits<Bar> {
public:
	typedef Bar Real;
	typedef Bar NonInteger;	
	typedef Bar Literal;
	typedef Bar Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Bar{};}
	auto static dummy_precision(){return Bar{};}
	auto static highest(){return Bar{};}
	auto static lowest(){return Bar{};}
	auto static digist10(){return 5;}
};
template<>
class Eigen::NumTraits<Baz> {
public:
	typedef Baz Real;
	typedef Baz NonInteger;	
	typedef Baz Literal;
	typedef Baz Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Baz{};}
	auto static dummy_precision(){return Baz{};}
	auto static highest(){return Baz{};}
	auto static lowest(){return Baz{};}
	auto static digist10(){return 5;}
};

template<>
class Eigen::NumTraits<Qux> {
public:
	typedef Qux Real;
	typedef Qux NonInteger;	
	typedef Qux Literal;
	typedef Qux Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Qux{};}
	auto static dummy_precision(){return Qux{};}
	auto static highest(){return Qux{};}
	auto static lowest(){return Qux{};}
	auto static digist10(){return 5;}
};

template<>
struct Eigen::ScalarBinaryOpTraits<Foo,Bar,Eigen::internal::scalar_product_op<Foo, Bar> >{
	typedef Baz ReturnType;
};

template<>
struct Eigen::ScalarBinaryOpTraits<Foo,Bar,Eigen::internal::scalar_sum_op<Baz, Baz> >{
	typedef Qux ReturnType;
};


void eigen_experiment() {
	Eigen::SparseMatrix<Foo> mymat(3,3);
	mymat.insert(0,0)=Foo{};
	mymat.insert(0,1)=Foo{};
	mymat.insert(1,0)=Foo{};
	mymat.insert(1,1)=Foo{};
	Eigen::SparseVector<Bar> myvec(3);
	myvec.insert(0)=Bar{};
	myvec.insert(1)=Bar{};
	Eigen::SparseVector<Baz> tmp=mymat*myvec;
	//Foo f; f=tmp;
	for (int k=0; k<mymat.outerSize(); ++k){
		for (decltype(mymat)::InnerIterator v(mymat,k); v;++v){
			printf("%d %d\n",v.row(),v.col());
		}
	}

	for (decltype(tmp)::InnerIterator v(tmp); v;++v){
		printf("%d\n",v.index());
	}
}

#pragma once
#include <Eigen/Sparse>

// setup for MatElem*VecElem->Prod
// goal is to add Prod+Prod->Acc,  Acc+=Prod,  Acc(Prod) Acc=0
class MatElem {public:MatElem(){} MatElem(int x){}};
class VecElem {public:VecElem(){} VecElem(int x){}};
class Prod {public:Prod(){} Prod(int x){}};
class Acc {public:Acc(){} Acc(const Prod&){printf("Acc(Prod)");} Acc(Prod&){printf("Acc(Prod)");} Acc(int x){}};
auto operator*(const MatElem& a,const VecElem& b){printf("MatElem*VecElem\n");return Prod{};}
auto operator+(const Prod& a,const Prod& b){printf("Prod+Prod\n");return Prod{};}
auto operator+=(Prod& a,const Prod& b){printf("Prod+=Prod\n");return a;}
auto operator+=(Acc& a,const Prod& b){printf("Acc+=Prod\n");return a;}
template<>
class Eigen::NumTraits<MatElem> {
public:
	typedef MatElem Real;
	typedef MatElem NonInteger;	
	typedef MatElem Literal;
	typedef MatElem Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return MatElem();}
	auto static dummy_precision(){return MatElem();}
	auto static highest(){return MatElem();}
	auto static lowest(){return MatElem();}
	auto static digist10(){return 5;}
};

template<>
class Eigen::NumTraits<VecElem> {
public:
	typedef VecElem Real;
	typedef VecElem NonInteger;	
	typedef VecElem Literal;
	typedef VecElem Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return VecElem{};}
	auto static dummy_precision(){return VecElem{};}
	auto static highest(){return VecElem{};}
	auto static lowest(){return VecElem{};}
	auto static digist10(){return 5;}
};
template<>
class Eigen::NumTraits<Prod> {
public:
	typedef Prod Real;
	typedef Prod NonInteger;	
	typedef Prod Literal;
	typedef Prod Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Prod{};}
	auto static dummy_precision(){return Prod{};}
	auto static highest(){return Prod{};}
	auto static lowest(){return Prod{};}
	auto static digist10(){return 5;}
};

template<>
class Eigen::NumTraits<Acc> {
public:
	typedef Acc Real;
	typedef Acc NonInteger;	
	typedef Acc Literal;
	typedef Acc Nested;
	enum {
		IsInteger=0,
		IsSigned=1,
		RequireInitialization=1,
		IsComplex=0,
		ReadCost=1,
		AddCost=1,
		MulCost=1
		
	};
	auto static epsilon(){return Acc{};}
	auto static dummy_precision(){return Acc{};}
	auto static highest(){return Acc{};}
	auto static lowest(){return Acc{};}
	auto static digist10(){return 5;}
};

template<>
struct Eigen::ScalarBinaryOpTraits<MatElem,VecElem,Eigen::internal::scalar_product_op<MatElem, VecElem> >{
	typedef Prod ReturnType;
};

template<>
struct Eigen::ScalarBinaryOpTraits<Prod,Prod,Eigen::internal::scalar_sum_op<Prod, Prod> >{
	typedef Prod ReturnType;
};


void eigen_experiment() {
	Eigen::SparseMatrix<MatElem> mymat(3,3);
	mymat.insert(0,0)=MatElem{};
	mymat.insert(0,1)=MatElem{};
	mymat.insert(1,0)=MatElem{};
	mymat.insert(1,1)=MatElem{};
	Eigen::SparseVector<VecElem> myvec(3);
	myvec.insert(0)=VecElem{};
	myvec.insert(1)=VecElem{};
	// Can't seem to do this with "Acc", even if supplying appropriate OpTraits etc above.
	Eigen::SparseVector<Prod> tmp=mymat*myvec;
	
	for (int k=0; k<mymat.outerSize(); ++k){
		for (decltype(mymat)::InnerIterator v(mymat,k); v;++v){
			printf("%d %d\n",v.row(),v.col());
		}
	}

	for (decltype(tmp)::InnerIterator v(tmp); v;++v){
		printf("%d\n",v.index());
	}
}

int main(int argc,const char**){
	eigen_experiment();
	return 0;
}
#pragma once
#include <Eigen/Sparse>

// setup for MatElem*VecElem->Prod
// goal is to add Prod+Prod->Acc,  Acc+=Prod,  Acc(Prod) Acc=0
// dummy types for each part of the process we want...
class MatElem {public:MatElem(){} MatElem(int x){}};
class VecElem {public:VecElem(){} VecElem(int x){}};
class Prod {public:Prod(){} Prod(int x){}};
class Acc {public:Acc(){} Acc(const Prod&){printf("Acc(Prod)\n");} Acc(Prod&){printf("Acc(Prod)\n");} Acc(int x){}};
// operators to combine information betwee matrix elems, input vector elems, and the output Accumulator, "Acc"
auto operator*(const MatElem& a,const VecElem& b){printf("MatElem*VecElem\n");return Prod{};}
auto operator+(const Prod& a,const Prod& b){printf("Prod+Prod\n");return Prod{};}
auto operator+=(Prod& a,const Prod& b){printf("Prod+=Prod\n");return a;}
auto operator+=(Acc& a,const Prod& b){printf("Acc+=Prod\n");return a;}

// Type Traits needed for Eigens' templates to work. needed for MatElem,VecElem,Prod,Acc
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
		// eigens templates apparently use these estimated costs 
 		// to decide when to combine expressions (wow)
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

// "operator traits" tell Eigen what types to create for its outputs . 
// (it can't just use decltype(inputs*..) ??)
template<>
struct Eigen::ScalarBinaryOpTraits<MatElem,VecElem,Eigen::internal::scalar_product_op<MatElem, VecElem> >{
	// MatElem*VecElem returns Product in our overload, but
	// telling it "Acc" here makes it sum and store those products into this Accumulator.
	// the mismatch sounds wrong but is actually what lets us have "a trivial type for the product"
	// and potential "nontrivial type for the accumulator (eg spike queue)
	typedef Acc ReturnType; 
};

template<>
struct Eigen::ScalarBinaryOpTraits<Prod,Prod,Eigen::internal::scalar_sum_op<Prod, Prod> >{
	typedef Acc ReturnType;
};


void eigen_experiment() {
	Eigen::SparseMatrix<MatElem> mymat(3,3);
	mymat.insert(0,0)=MatElem{};

	mymat.insert(0,2)=MatElem{};
	mymat.insert(1,0)=MatElem{};

	mymat.insert(1,2)=MatElem{};
	Eigen::SparseVector<VecElem> myvec(3);
	myvec.insert(0)=VecElem{};
	myvec.insert(1)=VecElem{};
	myvec.insert(2)=VecElem{};

	// run the matrix multiply, the prints in the operators will notify us when it calls each operator
	Eigen::SparseVector<Acc> result=mymat*myvec;
	
	// debug print the matrix filled indices
	for (int k=0; k<mymat.outerSize(); ++k){
		for (decltype(mymat)::InnerIterator v(mymat,k); v;++v){
			printf("%d %d\n",v.row(),v.col());
		}
	}

	// debug print the output vector filled indices
	for (decltype(result)::InnerIterator v(result); v;++v){
		printf("%d\n",v.index());
	}
}

int main(int argc,const char**){
	eigen_experiment();
	return 0;
}





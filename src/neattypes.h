#pragma once

#ifndef NEATTYPES_H__
#define NEATTYPES_H__

// Some common types that must be in their own header so they can be used by the
// CUDA compiler (C++11 features not currently supported).
namespace NEAT {

//    typedef float real_t;
    typedef double real_t;

    typedef unsigned char uchar;
    typedef unsigned short ushort;

    #define NODES_MAX USHRT_MAX
    #define LINKS_MAX USHRT_MAX

    typedef unsigned short node_size_t;
    typedef unsigned short link_size_t;

	enum nodetype {
        NT_BIAS = 0,
		NT_SENSOR = 1,
		NT_OUTPUT = 2,
		NT_HIDDEN = 3
	};

#if __cplusplus < 199711L
    class OrganismEvaluation {
    public:
        real_t fitness;
        real_t error;

        void reset() {fitness = error = 0.0;}
    };
#else
    struct OrganismEvaluation {
        real_t fitness;
        real_t error;

        void reset() {fitness = error = 0.0;}
    };
#endif

#undef __in
#undef __out
#undef __inout
    #define __in const
    #define __out
    #define __inout
}

#endif // #ifndef NEATTYPES_H__

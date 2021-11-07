#pragma once

#ifndef GENOMEMANAGER_H__
#define GENOMEMANAGER_H__

#if __cplusplus >= 199711L
#include "neat.h"
#include "rng.h"
#else
#endif

namespace NEAT {

    class Genome;

    class GenomeManager {
    public:
        static GenomeManager *create();

        virtual ~GenomeManager() {}

        virtual std::unique_ptr<Genome> make_default() = 0;

        virtual std::vector<std::unique_ptr<Genome>> create_seed_generation(size_t ngenomes,
                                                                            class rng_t rng,
                                                                            size_t ntraits,
                                                                            size_t ninputs,
                                                                            size_t noutputs,
                                                                            size_t nhidden) = 0;

        virtual bool are_compatible(Genome &genome1,
                                    Genome &genome2) = 0;

        virtual void clone(Genome &orig, Genome &clone) = 0;

        virtual void mate(Genome &genome1,
                          Genome &genome2,
                          Genome &offspring,
                          real_t fitness1,
                          real_t fitness2) = 0;
 
        enum MutationOperation {
            MUTATE_OP_WEIGHTS,
            MUTATE_OP_STRUCTURE,
            MUTATE_OP_ANY
        };
        virtual void mutate(Genome &genome,
                            MutationOperation op = MUTATE_OP_ANY) = 0;

        virtual void finalize_generation(bool new_fittest) = 0;
    };

}
#endif // #ifndef GENOMEMANAGER_H__


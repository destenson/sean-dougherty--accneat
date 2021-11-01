/*
 Copyright 2001 The University of Texas at Austin

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
#pragma once 

#include "organismsbuffer.h"
#include "population.h"
#include "speciesorganism.h"

namespace NEAT {

	class SpeciesPopulation : public Population {
	public:
		// Construct off of a single spawning Genome 
		SpeciesPopulation(rng_t rng,
                          std::vector<std::unique_ptr<Genome>> &seeds);
		virtual ~SpeciesPopulation();

        virtual size_t size() override;
        virtual class Organism *get(size_t index) override;
        virtual std::unique_ptr<Organism> make_copy(size_t index) override;

		virtual void next_generation() override;
		virtual void verify() override;

		virtual void write(std::ostream& out) override;

    private:
		void spawn();
		void speciate();

        size_t norgs;
        int generation;
        OrganismsBuffer<SpeciesOrganism> orgs;

        std::vector<class Species*> species;  // Species in the SpeciesPopulation. Note that the species should comprise all the genomes 

		// ******* Member variables used during reproduction *******
		int last_species;  //The highest species number

		// ******* When do we need to delta code? *******
		real_t highest_fitness;  //Stagnation detector
		int highest_last_changed; //If too high, leads to delta coding
        void compute_fitnesses();
    };

} // namespace NEAT

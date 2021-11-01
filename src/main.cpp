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
#include "std.h" // Must be included first. Precompiled header with standard library includes.
#ifdef __linux__
#include <unistd.h>
#endif
#include "experiment.h"
#include "neat.h"
#include "rng.h"
#include "util.h"

using namespace NEAT;
using namespace std;

#define DEFAULT_RNG_SEED 1
#define DEFAULT_MAX_GENS 10000

void usage() {
    cerr << "usage: neat [OPTIONS]... experiment_name" << endl;
    cerr << endl;
    cerr << "experiment names: ";
    auto names = Experiment::get_names();
    for(size_t i = 0; i < names.size(); i++) {
        if(i != 0)
            cerr << ", ";
        cerr << names[i];
    }
    cerr << endl;
    cerr << endl;

    cerr << "OPTIONS" << endl;
    cerr << "  -f                   Force deletion of any data from previous run." << endl;
    cerr << "  -c num_experiments   (default=" << env->num_runs << ")" << endl;
    cerr << "  -r RNG_seed          (default=" << DEFAULT_RNG_SEED << ")" << endl;
    cerr << "  -n population_size   (default=" << env->pop_size << ")" << endl;
    cerr << "  -x max_generations   (default=" << DEFAULT_MAX_GENS << ")" << endl;
    cerr << "  -s search_type       {phased, blended, complexify} (default=phased)" << endl;


    exit(1);
}

template<typename T>
T parse_enum(const char *opt, string str, map<string,T> vals) {
    auto it = vals.find(str);
    if(it == vals.end()) {
        error("Invalid value for " << opt << ": " << str);
    }
    return it->second;
}

int parse_int(const char *opt, const char *str) {
    try {
        return stoi(str);
    } catch(...) {
        error("Expecting integer argument for " << opt << ", found '" << str << "'.");
    }
}

int main(int argc, char *argv[]) {

    int rng_seed = DEFAULT_RNG_SEED;
    int maxgens = DEFAULT_MAX_GENS;
    bool force_delete = false;

#ifdef __linux__
    {
        int opt;
        while( (opt = getopt(argc, argv, "fc:r:p:g:n:x:s:")) != -1) {
            switch(opt) {
            case 'f':
                force_delete = true;
                break;
            case 'c':
                env->num_runs = parse_int("-c", optarg);
                break;
            case 'r':
                rng_seed = parse_int("-r", optarg);
                break;
            case 'n':
                env->pop_size = parse_int("-n", optarg);
                break;
            case 'x':
                maxgens = parse_int("-x", optarg);
                break;
            case 's':
                env->search_type = parse_enum<GeneticSearchType>("-s", optarg, {
                        {"phased", GeneticSearchType::PHASED},
                        {"blended", GeneticSearchType::BLENDED},
                        {"complexify", GeneticSearchType::COMPLEXIFY}
                    });
                break;
            default:
                error("Invalid option: -" << (char)opt);
            }
        }
    }

    int nargs = argc - optind;
    if(nargs == 0) {
        usage();
    } else if(nargs > 1) {
        error("Unexpected argument: " << argv[optind+1]);
    }

    if(force_delete) {
        sh("rm -rf experiment_*");
    } else if(exists("experiment_1")) {
        error("Already exists: experiment_1.\nMove your experiment directories or use -f to delete them automatically.")
    }

    if(env->search_type == GeneticSearchType::BLENDED) {
        env->mutate_delete_node_prob *= 0.1;
        env->mutate_delete_link_prob *= 0.1;
    }

    const char *experiment_name = argv[optind++];

    Experiment *exp = Experiment::get(experiment_name);
    if(exp == nullptr) {
        trap("No such experiment: " << experiment_name);
    }
#else
//    sh("bash -c 'rm -f experiment_*'");
    env->num_runs = 10;
    env->pop_size = 5000;

    env->search_type = GeneticSearchType::COMPLEXIFY;
    env->population_type = PopulationType::SPECIES;
    env->genome_type = GenomeType::INNOV;

    env->trait_param_mut_prob = 0.5;
    env->trait_mutation_power = 1.0; // Power of mutation on a signle trait param
    env->linktrait_mut_sig = 1.0; // Amount that mutation_num changes for a trait change inside a link
    env->nodetrait_mut_sig = 0.5; // Amount a mutation_num changes on a link connecting a node that changed its trait
    env->weight_mut_power = 1.8; // The power of a linkweight mutation

    env->recur_prob = 0.05; // Prob. that a link mutation which doesn't have to be recurrent will be made recurrent

    // These 3 global coefficients are used to determine the formula for
    // computating the compatibility between 2 genomes.  The formula is:
    // disjoint_coeff*pdg+excess_coeff*peg+mutdiff_coeff*mdmg.
    // See the compatibility method in the Genome class for more info
    // They can be thought of as the importance of disjoint Genes,
    // excess Genes, and parametric difference between Genes of the
    // same function, respectively.
    env->disjoint_coeff = 1.0;
    env->excess_coeff = 1.0;
    env->mutdiff_coeff = 3.0;

    // This global tells compatibility threshold under which two Genomes are considered the same species
    env->compat_threshold = 10.0;

    env->age_significance = 1.0; // How much does age matter?
    env->survival_thresh = 0.4; // Percent of ave fitness for survival
    env->mutate_only_prob = 0.25; // Prob. of a non-mating reproduction
    env->mutate_random_trait_prob = 0.1;
    env->mutate_link_trait_prob = 0.1;
    env->mutate_node_trait_prob = 0.1;
    env->mutate_link_weights_prob = 0.8;
    env->mutate_toggle_enable_prob = 0.1;
    env->mutate_gene_reenable_prob = 0.05;
    env->mutate_add_node_prob = 0.01;
    env->mutate_delete_node_prob = 0.01;
    env->mutate_add_link_prob = 0.3;
    env->mutate_delete_link_prob = 0.3;
    env->mutate_add_link_reenables = false;
    env->interspecies_mate_rate = 0.001; // Prob. of a mate being outside species
    env->mate_multipoint_prob = 0.6;
    env->mate_only_prob = 0.2; // Prob. of mating without mutation
    env->recur_only_prob = 0.2;  // Probability of forcing selection of ONLY links that are naturally recurrent

    env->dropoff_age = 15;  // Age where Species starts to be penalized
    env->newlink_tries = 20;  // Number of tries mutate_add_link will attempt to find an open link
    env->print_every = 1000; // Tells to print population to file every n generations

    Experiment *exp = Experiment::get("xor");
    if(exp == nullptr) {
        trap("No such experiment: xor");
    }
#endif

    rng_t rng{rng_seed};
    exp->run(rng, maxgens);

#ifdef __linux__
    string cmd("find -name fittest_* -print0 | xargs -0 cat > ");
    cmd += experiment_name;
    cmd += string("-") + to_string(env->num_runs) + string(".results");
    sh(cmd.c_str());
    sh("rm -rf experiment_*");
#endif

    return(0);
}


cmake_minimum_required(VERSION 3.16)
project(accneat)

#set(CMAKE_CXX_STANDARD 98)

include_directories(src)
include_directories(src/experiments)
include_directories(src/experiments/maze)
include_directories(src/experiments/static)
include_directories(src/innovgenome)
include_directories(src/network)
include_directories(src/network/cpu)
include_directories(src/network/cuda)
include_directories(src/species)
include_directories(src/util)

set(CMAKE_CXX_STANDARD 11)

add_library(accneat
#        src/experiments/maze/maze.cpp
#        src/experiments/maze/mazeevaluator.h
        src/experiments/static/cfg.cpp
        src/experiments/static/regex.cpp
        src/experiments/static/sequence.cpp
        src/experiments/static/staticevaluator.h
        src/experiments/static/staticexperiment.h
        src/experiments/static/xor.cpp
        src/experiments/evaluatorexperiment.h
        src/experiments/experiment.cpp
        src/experiments/experiment.h
        src/innovgenome/innovation.cpp
        src/innovgenome/innovation.h
        src/innovgenome/innovgenome.cpp
        src/innovgenome/innovgenome.h
        src/innovgenome/innovgenomemanager.cpp
        src/innovgenome/innovgenomemanager.h
        src/innovgenome/innovlinkgene.cpp
        src/innovgenome/innovlinkgene.h
        src/innovgenome/innovnodegene.cpp
        src/innovgenome/innovnodegene.h
        src/innovgenome/innovnodelookup.h
        src/innovgenome/protoinnovlinkgene.h
        src/innovgenome/recurrencychecker.h
        src/innovgenome/trait.cpp
        src/innovgenome/trait.h
        src/network/cpu/cpunetwork.cpp
        src/network/cpu/cpunetwork.h
        src/network/cpu/cpunetworkexecutor.h
        src/network/cuda/cudanetwork.cu
        src/network/cuda/cudanetwork.h
        src/network/cuda/cudanetworkbatch.h
        src/network/cuda/cudanetworkexecutor.h
        src/network/cuda/cudanetworkkernel.h
        src/network/cuda/cudautil.h
        src/network/network.h
        src/network/networkexecutor.h
        src/species/species.cpp
        src/species/species.h
        src/species/speciesorganism.cpp
        src/species/speciesorganism.h
        src/species/speciespopulation.cpp
        src/species/speciespopulation.h
        src/util/map.cpp
        src/util/map.h
        src/util/organismsbuffer.h
        src/util/resource.cpp
        src/util/resource.h
        src/util/rng.cpp
        src/util/rng.h
        src/util/stats.h
        src/util/std.h
        src/util/std.hxx
        src/util/timer.cpp
        src/util/timer.h
        src/util/util.cpp
        src/util/util.h
        src/genome.h
        src/genomemanager.cpp
        src/genomemanager.h
#        src/experiments/maze/mazeevaluator.cxx
        src/experiments/static/staticevaluator.cxx
        src/neat.cpp
        src/neat.h
        src/neattypes.h
        src/organism.cpp
        src/organism.h
        src/population.cpp
        src/population.h)

#install(TARGETS test DESTINATION lib)
#install(TARGETS test DESTINATION lib)

#add_dependencies(accneat accneat-static)

#add_executable(acc-neat
#        src/main.cpp
#        src/neat.cpp
#        src/experiments/maze/mazeevaluator.cxx
#        src/experiments/static/staticevaluator.cxx
#        )
#
#add_dependencies(acc-neat accneat accneat-static)

#target_link_libraries(accneat PUBLIC )

cmake_minimum_required(VERSION 3.16)
project(accneatlib-staticexperiments)

#set(CMAKE_CXX_STANDARD 98)

include_directories(../../src)
include_directories(../../src/experiments)
include_directories(../../src/experiments/static)
include_directories(../../src/network)
include_directories(../../src/network/cpu)
include_directories(../../src/species)
include_directories(../../src/util)

set(CMAKE_CXX_STANDARD 98)

add_library(accneatlib-staticexperiments
#        ../../src/experiments/maze/maze.cpp
#        ../../src/experiments/maze/mazeevaluator.h
        ../../src/experiments/static/staticevaluator.h
        ../../src/experiments/static/staticexperiment.h
        ../../src/experiments/evaluatorexperiment.h
#        ../../src/experiments/experiment.cpp
        ../../src/experiments/experiment.h
#        ../../src/experiments/maze/mazeevaluator.cxx
#        ../../src/experiments/static/staticevaluator.cxx
        ../../src/network/network.h
        ../../src/network/networkexecutor.h
        ../../src/network/cpu/cpunetwork.h
        ../../src/network/cpu/cpunetworkexecutor.h
        ../../src/util/std.hxx
        ../../src/genomemanager.h
        ../../src/neattypes.h
        )

install(TARGETS accneatlib-staticexperiments DESTINATION lib)

set_property(TARGET accneatlib-staticexperiments PROPERTY CXX_STANDARD 98)
set_property(TARGET accneatlib-staticexperiments PROPERTY LANGUAGE CXX)
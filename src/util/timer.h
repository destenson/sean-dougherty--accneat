#pragma once

#ifndef UTIL_TIMER_H__
#define UTIL_TIMER_H__

namespace NEAT {
    class Timer {
        static std::vector<Timer *> timers;

        const char *_name;
        size_t _n = 0;
        double _total = 0.0;
        double _min = 0.0;
        double _max = 0.0;
        double _start = 0.0;
        double _recent = 0.0;
    public:
        Timer(const char *name);
        ~Timer();
    
        void start();
        void stop();

        static void report();
    };
}

#endif // #ifndef UTIL_TIMER_H__

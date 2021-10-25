#include "std.h" // Must be included first. Precompiled header with standard library includes.
#include "timer.h"
#include <assert.h>
#ifdef __linux__
#include <sys/time.h>
#endif

using namespace NEAT;
using namespace std;

vector<Timer *> Timer::timers;

#ifdef __linux__
static double seconds() {
    struct timeval tv;
    gettimeofday( &tv, NULL );

    return double(tv.tv_sec + tv.tv_usec/1000000.0);
}
#endif

Timer::Timer(const char *name) : _name(name) {
    timers.push_back(this);
}

Timer::~Timer() {
    timers.erase(find(timers.begin(), timers.end(), this));
}

void Timer::start() {
#ifdef __linux__
    assert(_start == 0.0);

    _start = seconds();
#endif
}

void Timer::stop() {
#ifdef __linux__
    assert(_start != 0.0);

    double t = seconds() - _start;
#else
    double t = 0.0000001;
#endif
    _recent = t;
    _start = 0.0;

    if(_n == 0) {
        _min = _max = t;
    } else {
        _min = min(_min, t);
        _max = max(_max, t);
    }
    _total += t;
    _n++;
}

void Timer::report() {
    for(Timer *t: timers) {
        cout << t->_name << ": n=" << t->_n << ", recent=" << t->_recent << ", mean=" << (t->_total / t->_n) << ", min=" << t->_min << ", max=" << t->_max << ", total=" << t->_total << endl;
    }
}

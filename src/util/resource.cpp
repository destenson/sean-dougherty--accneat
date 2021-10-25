#include "std.h" // Must be included first. Precompiled header with standard library includes.
#include "resource.h"
#include "util.h"
#ifdef __linux__
#include <unistd.h>
#endif

using namespace std;

namespace NEAT {

    string find_resource(const string &name) {
#ifdef __linux__
        char home[1024];
        ssize_t rc = readlink("/proc/self/exe", home, sizeof(home));
        if(rc < 1) {
            error("Couldn't resolve /proc/self/exe! Is this Linux?");
        }
        if(rc == sizeof(home)) {
            error("Possible buffer overrun.");
        }

        *strrchr(home, '/') = 0;

        return string(home) + "/res/" + name;
#else
        return string("");
#endif
    }

}

#include "std.h" // Must be included first. Precompiled header with standard library includes.
#include "util.h"
#ifdef __linux__
#include <sys/types.h>
#include <sys/stat.h>
#else
#include <direct.h>
#include <io.h>
#endif

using namespace std;

void mkdir(const string &path) {
    int status;
#ifdef __linux__
    status = ::mkdir(path.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH);
#else
    status = _mkdir(path.c_str());
#endif
    if(0 != status) {
        char buf[2048];
        sprintf(buf, "Failed making directory '%s'", path.c_str());
        perror(buf);
        exit(1);
    }
}

bool exists(const std::string &path) {
#ifdef __linux__
    struct stat buffer;
    return (stat (path.c_str(), &buffer) == 0);
#else
    return _access(path.c_str(), 0) == 0;
#endif
}

vector<string> permute_repeat(const string &letters,
                              size_t len) {
    vector<string> result;
    string buf;
    
    struct local {
        static void __permute(const string &letters,
                              size_t depth,
                              size_t len,
                              vector<string> &result,
                              string &buf) {
            if(depth == len) {
                result.push_back(buf);
            } else {
                for (size_t i = 0; i < letters.size(); ++i) {
                    buf.append(letters, i, 1);
                    __permute(letters, depth+1, len, result, buf);
                    buf.erase(buf.size() - 1);
                }
            }
        }
    };

    local::__permute(letters, 0, len, result, buf);
    
    return result;
}

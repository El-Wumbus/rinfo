#include "cpu.h"

#ifdef __APPLE__
#include <string.h>
#include <sys/sysctl.h>
#include <sys/types.h>
#include <stdlib.h>

int macos_get_caller(size_t pid, char* buffer, size_t buffer_len)
{
    int argmax;
    size_t argmax_size;
    char *procargs, *cp, *thiscmd;

    int mib[3] = {CTL_KERN, KERN_ARGMAX};

    argmax_size = sizeof(argmax);
    if (sysctl(mib, 2, &argmax, &argmax_size, NULL, 0) == -1) {
        goto _end;
    }

    procargs = malloc(argmax);
    if (procargs == NULL) {
        goto _end;
    }

    mib[0] = CTL_KERN;
    mib[1] = KERN_PROCARGS;
    mib[2] = pid;

    argmax_size = (size_t)argmax;
    if (sysctl(mib, 3, procargs, &argmax_size, NULL, 0) == -1) {
        free(procargs);
        goto _end;
    }

    /* Find the end of the first argument string */
    for (cp = procargs; *cp != '\0'; cp++)
        ;

    /* Find the start of the second argument string */
    for (; cp < &procargs[argmax_size] && *cp == '\0'; cp++)
        ;

    /* Find the end of the second argument string */
    for (thiscmd = cp; cp < &procargs[argmax_size] && *cp != '\0'; cp++)
        ;

    /* Strip off any path that was specified */
    for (; thiscmd < cp && *thiscmd != '/'; thiscmd++)
        ;

    if (thiscmd == cp) {
        free(procargs);
        goto _end;
    }

    size_t len = cp - thiscmd;
    if (buffer_len <= len) {
        free(procargs);
        return -2;
    }

    memcpy(buffer, thiscmd, len);
    buffer[len] = '\0';

    free(procargs);
    return 0;

_end:
    return 0;
}

_end:

    return 0;
}
#else
int macos_get_caller(size_t pid, char* buffer)
{
    return -1;
}
#endif